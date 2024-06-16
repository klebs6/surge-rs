/*!
  | This code is an implementation of an LFO
  | (Low-Frequency Oscillator) module for
  | a digital audio workstation (DAW).
  |
  | The LFO generates a periodic waveform at a low
  | frequency (typically less than 20 Hz) and can
  | be used to modulate various parameters of the
  | audio signal, such as volume, pitch, or filter
  | cutoff frequency.
  |
  | The `Lfo` struct contains various properties
  | and methods that define the behavior of the
  | LFO.
  |
  | The `new()` method is a constructor that
  | creates a new instance of the LFO and
  | initializes its properties. The
  | `init_phase_from_start_phase()` method
  | initializes the phase of the LFO based on
  | a starting phase value.
  |
  */

crate::ix!();

impl Lfo {

    /// The new() method creates a new instance of
    /// the LFO with the given time unit and
    /// tables handles.
    ///
    pub fn new( 
        time_unit: TimeUnitHandle,
        tables:    TablesHandle) -> Self 
    {
        Self {
            params:             LfoParam::new_runtime(),
            output:             0.0,
            stepsequencer:      StepSequencer::default(),
            phase_initialized:  false,
            env_val:            0.0,
            env_state:          LfoEnvState::Delay,
            retrigger_feg:      false,
            retrigger_aeg:      false,
            phase:              0.0,
            target:             0.0,
            noise:              0.0,
            noised1:            0.0,
            env_phase:          0.0,
            ratemult:           1.0,
            env_releasestart:   0.0,
            iout:               0.0,
            wf_history:         [0.0; 4],
            step:               0,
            shuffle_id:         0,
            sine:               QuadrOsc::default(),
            time_unit:          time_unit.clone(),
            tables:             tables.clone(),
            enabled:            true
        }
    }

    /// The init_phase_from_start_phase() method
    /// initializes the LFO phase based on
    /// a starting phase value.
    ///
    #[inline] pub fn init_phase_from_start_phase(&mut self) {

        // Set the LFO phase to the starting phase value.
        self.phase = pvalf![self.params[LfoParam::StartPhase]];

        self.phase_initialized = true;

        // Make sure the phase is between 0 and 1.
        while self.phase < 0.0 { self.phase += 1.0 ; }
        while self.phase > 1.0 { self.phase -= 1.0 ; }
    }
}
