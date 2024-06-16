crate::ix!();

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct SurgeTuner {
    pub current_tuning:  Align16<SurgeTuning>,
    pub current_mapping: Align16<KeyboardMapping>,
    pub current_scale:   Align16<Scale>,
    pub tables:          Align16<TuningTables>,
    pub srunit:          SampleRateHandle,
}

impl SurgeTuner {

    pub fn new(srunit: &SampleRateHandle) -> Self {
        Self {
            current_tuning:  Align16(SurgeTuning::default()),
            current_mapping: Align16(KeyboardMapping::default()),
            current_scale:   Align16(Scale::default()),
            tables:          Align16(TuningTables::new(srunit)),
            srunit:          srunit.clone(),
        }
    }

    #[inline] pub fn default_scale_count<T>() -> T 
    where T: TryFrom<i8>, <T as TryFrom<i8>>::Error: Debug
    {
        T::try_from(12).unwrap()
    }
}

impl ScaleNote for SurgeTuner {

    #[inline] fn scale_constant_note(&self) -> i32 { 
        self.current_mapping.tuning_constant_note
    }

    #[inline] fn scale_constant_pitch(&self) -> f32 { 
        self.current_tuning.pitch
    }
}

impl Initialize for SurgeTuner {
    fn init(&mut self) {
        self.current_tuning.init();
        self.current_mapping.init();
        self.current_scale.init();
        self.tables.init();
    }
}

impl Note2Pitch for SurgeTuner {

    #[inline] fn n2p_tuningctr<T: MyFloat>(&self, x: T) -> T 
    {
        let scale_constant_note      = T::from(self.scale_constant_note()).unwrap();
        let scale_constant_pitch_inv = T::from(self.scale_constant_pitch_inv()).unwrap();

        self.n2p::<T,false>( x + scale_constant_note ) * scale_constant_pitch_inv
    }

    #[inline] fn n2pinv_tuningctr<T: MyFloat>(&self, x: T) -> T 
    {
        let scale_constant_note      = T::from(self.scale_constant_note()).unwrap();
        let scale_constant_pitch_inv = T::from(self.scale_constant_pitch_inv()).unwrap();

        self.n2pinv::<T,false>( x + scale_constant_note ) * scale_constant_pitch_inv
    }

    fn n2p<T: MyFloat, const IGNORE_TUNING: bool>(&self, mut x: T) -> T 
    {
        x += T::from(256.0).unwrap();
        let f: f64 = x.into();
        let mut e: i64 = f as i64;
        let lerpx: T = x - (T::from(e).unwrap());

        if e > 0x1fe {
            e = 0x1fe;
        }

        let table = match IGNORE_TUNING {
            true => &self.tables.table_pitch_ignoring_tuning,
            false => &self.tables.table_pitch,
        };

        let aidx = (e & 0x1ff) as usize;
        let bidx = ((e + 1) & 0x1ff) as usize;

        let lerpa = T::from(table[aidx]).unwrap();
        let lerpb = T::from(table[bidx]).unwrap();

        lerp(lerpx,lerpa,lerpb)
    }

    fn n2pinv<T: MyFloat, const IGNORE_TUNING: bool>(&self, mut x: T) -> T 
    {
        x += T::from(256.0).unwrap();
        let f: f64 = x.into();
        let mut e: i64 = f as i64;
        let lerpx: T = x - (T::from(e).unwrap());

        if e > 0x1fe {
            e = 0x1fe;
        }

        let aidx = (e & 0x1ff) as usize;
        let bidx = ((e + 1) & 0x1ff) as usize;

        let table = match IGNORE_TUNING {
            true => &self.tables.table_pitch_inv_ignoring_tuning,
            false => &self.tables.table_pitch_inv,
        };

        let lerpa = T::from(table[aidx]).unwrap();
        let lerpb = T::from(table[bidx]).unwrap();

        lerp(lerpx,lerpa,lerpb)
    }

    fn note_to_omega<T: MyFloat, const IGNORE_TUNING: bool>(&self, mut x: T) -> (T, T) 
    {
        x += T::from(256.0).unwrap();

        let f: f64 = x.into();
        let mut e: i64 = f as i64;

        let lerpx: T = x - (T::from(e).unwrap());

        if e > 0x1fe {
            e = 0x1fe;
        } else if e < 0 {
            e = 0;
        }

        let table = match IGNORE_TUNING {
            true => self.tables.table_note_omega_ignoring_tuning.view(),
            false => self.tables.table_note_omega.view(),
        };

        let aidx = (e & 0x1ff) as usize;
        let bidx = ((e + 1) & 0x1ff) as usize;

        let sinu_lerpa = T::from(table[[0,aidx]]).unwrap();
        let sinu_lerpb = T::from(table[[0,bidx]]).unwrap();

        let cosi_lerpa = T::from(table[[1, aidx]]).unwrap();
        let cosi_lerpb = T::from(table[[1, bidx]]).unwrap();

        let sinu = lerp(lerpx, sinu_lerpa, sinu_lerpb);
        let cosi = lerp(lerpx, cosi_lerpa, cosi_lerpb);
        (sinu, cosi)
    }

    #[inline] fn pitch2omega<T: MyFloat>(&self, x: T) -> T {

        let sros64_inv = self.srunit.dsamplerate_os_inv();

        T::from(PI).unwrap() * 
            T::from(NOTE_FREQ_C0).unwrap() * 
            self.n2p::<T,false>(x) * 
            T::from(sros64_inv).unwrap()
    }
}

impl CurrentScaleCount for SurgeTuner {

    #[inline] fn current_scale_count<T>(&self) -> T 
    where 
        T: TryFrom<i8>,
        <T as std::convert::TryFrom<i8>>::Error: std::fmt::Debug 
    {
        let val = self.current_scale.count;
        assert!(val & 127 == val);
        T::try_from(val as i8).unwrap()
    }
}

impl CurrentScale for SurgeTuner {
    #[inline] fn current_scale(&self) -> Scale {
        self.current_scale.0.clone()
    }
}

impl CurrentTuning for SurgeTuner {
    #[inline] fn current_tuning(&self) -> SurgeTuning {
        self.current_tuning.0.clone()
    }
}

impl CurrentTuningIsStandard for SurgeTuner {
    #[inline] fn current_tuning_is_standard(&self) -> bool {
        self.current_tuning.is_standard_tuning
    }
}

impl CurrentMappingIsStandard for SurgeTuner {
    #[inline] fn current_mapping_is_standard(&self) -> bool {
        self.current_mapping.is_standard_mapping
    }
}

impl CurrentScaleRawContents for SurgeTuner {
    #[inline] fn current_scale_raw_contents(&self) -> TuningData {
        self.current_scale.raw_text.clone()
    }
}

impl CurrentMappingRawContents for SurgeTuner {
    #[inline] fn current_mapping_raw_contents(&self) -> MappingData {
        self.current_mapping.raw_text.clone()
    }
}

impl GetTablePitch for SurgeTuner {

    #[inline] fn get_tablepitch<IDX>(&self, idx: IDX) -> f64 
    where 
        IDX: TryInto<usize>,
        <IDX as std::convert::TryInto<usize>>::Error: std::fmt::Debug
    {
        let idx: usize = idx.try_into().unwrap(); 
        self.tables.table_pitch[idx]
    }
}
