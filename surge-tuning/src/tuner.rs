ix!();

use crate::{
    SurgeTuning,
    KeyboardMapping,
    Scale,
    ScaleNote,
    Note2Pitch,
    TuningTables,
};

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct SurgeTuner<'sr> {
    pub current_tuning:  Align16<SurgeTuning>,
    pub current_mapping: Align16<KeyboardMapping>,
    pub current_scale:   Align16<Scale>,
    pub tables:          Align16<TuningTables<'sr>>,
    pub srunit:          SampleRateHandle<'sr>,
}

impl SurgeTuner<'sr> {

    pub fn new(srunit: &'sr SampleRateHandle<'sr>) -> Self {
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

impl ScaleNote for SurgeTuner<'sr> {

    #[inline] fn scale_constant_note(&self) -> i32 { 
        self.current_mapping.tuning_constant_note
    }

    #[inline] fn scale_constant_pitch(&self) -> f32 { 
        self.current_tuning.pitch
    }
}

impl Init for SurgeTuner<'sr> {
    fn init(&mut self) {
        self.current_tuning.init();
        self.current_mapping.init();
        self.current_scale.init();
        self.tables.init();
    }
}

impl Note2Pitch for SurgeTuner<'sr> {

    #[inline] fn n2p_tuningctr<T: MyFloat>(&self, x: T) -> T 
    {
        let scale_constant_note      = T::from(self.scale_constant_note()).unwrap();
        let scale_constant_pitch_inv = T::from(self.scale_constant_pitch_inv()).unwrap();

        self.n2p::<false,T>( x + scale_constant_note ) * scale_constant_pitch_inv
    }

    #[inline] fn n2pinv_tuningctr<T: MyFloat>(&self, x: T) -> T 
    {
        let scale_constant_note      = T::from(self.scale_constant_note()).unwrap();
        let scale_constant_pitch_inv = T::from(self.scale_constant_pitch_inv()).unwrap();

        self.n2pinv::<false,T>( x + scale_constant_note ) * scale_constant_pitch_inv
    }

    fn n2p<const IGNORE_TUNING: bool, T: MyFloat>(&self, mut x: T) -> T 
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

    fn n2pinv<const IGNORE_TUNING: bool, T: MyFloat>(&self, mut x: T) -> T 
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

    fn note_to_omega<const IGNORE_TUNING: bool, T: MyFloat>(&self, mut x: T) -> (T, T) 
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
            self.n2p::<false,T>(x) * 
            T::from(sros64_inv).unwrap()
    }
}
