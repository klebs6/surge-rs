crate::ix!();

pub struct SynthEnvironment<'a> {
    input:           &'a mut SynthInputHandle,
    output:          &'a mut SynthOutputHandle<BLOCK_SIZE>,
    tuner:           &'a mut TunerHandle,
    tables:          &'a mut TablesHandle,
    srunit:          &'a mut SampleRateHandle,
    timeunit:        &'a mut TimeUnitHandle,
    hold_pedal_unit: &'a mut HoldPedalUnitHandle,
    midi_unit:       &'a mut MIDIUnitHandle,
    mpe_unit:        &'a mut MPEUnitHandle,
    synth_in:        &'a mut SynthInputHandle,
}

impl<'plugin_layer> SurgeSynthesizer<'plugin_layer> {

    pub fn new_default_patch<'synth_out>(environment: &SynthEnvironment<'synth_out>) -> Result<Box<SurgePatch>,SurgeError> {
        Ok(Box::new(SurgePatch::new(
                SceneConstructorHandles{
                    timeunit:         environment.timeunit, 
                    tables:           environment.tables, 
                    tuner:            environment.tuner, 
                    srunit:           environment.srunit,
                    hold_pedal_unit:  environment.hold_pedal_unit, 
                    midi_unit:        environment.midi_unit, 
                    mpe_unit:         environment.mpe_unit,
                    synth_in:         environment.synth_in
                }
        )?))
    }

    pub fn new_fx_unit<'synth_out>(environment: &SynthEnvironment<'synth_out>) -> Result<FXUnit,SurgeError> {
        Ok(FXUnit::new(
            environment.tuner,
            environment.tables,
            environment.timeunit,
            environment.srunit
        )?)
    }

    pub fn new_default<'synth_out>(environment: SynthEnvironment<'synth_out>) -> Result<Self,SurgeError> {

        let mut x = Self {
            synth_out:                 environment.output.clone(),
            synth_in:                  environment.input.clone(),
            tuner:                     environment.tuner.clone(),
            tables:                    environment.tables.clone(),
            srunit:                    environment.srunit.clone(),
            timeunit:                  environment.timeunit.clone(),
            amp:                       Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            amp_mute:                  Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            controllers:               [0; N_CUSTOMCONTROLLERS],
            halfband_in:               HalfRateFilterSSE::default(),
            plugin_layer:              PluginLayer::default(),
            cc0:                       0,
            pch:                       0,
            controller:                SynthControl::default(),
            fx_unit:                   Self::new_fx_unit(&environment)?,
            hold_pedal_unit:           environment.hold_pedal_unit.clone(),
            midi_unit:                 environment.midi_unit.clone(),
            mpe_unit:                  environment.mpe_unit.clone(),
            patch_loaded:              false,
            patchid:                   None,
            current_category_id:       None,
            patchid_queue:             None,
            active_patch:              Self::new_default_patch(&environment)?,
            patches:                   vec![],
            patch_categories:          vec![],
            active_patch_category:     vec![],
            first_3p_category:         0,
            first_user_category:       0,
            patch_ordering:            vec![],
            patch_category_ordering:   vec![],
            audio_processing_active:   false,
        };

        x.initialize_customcontrollers();
        x.initialize_midi_controllers();
        Ok(x)
    }

    pub fn initialize_customcontrollers(&mut self) {
        for idx in 0..N_CUSTOMCONTROLLERS {
            self.controllers[idx] = 41 + idx as i32;
        }
    }

    pub fn initialize_midi_controllers(&mut self) {
        self.midi_unit.load_midi_controllers();
    }
}
