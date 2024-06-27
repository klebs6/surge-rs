crate::ix!();

/// The ADSR envelope describes the change of
/// a sound's amplitude over time. 
///
/// This is a commonly used component in
/// electronic music synthesis. 
///
/// It contains various fields that store
/// information about the envelope's state,
///
#[derive(Debug)]
#[modulation_source_control]
#[name("envelope")]
pub struct AdsrEnvelope {

    /// an array of ADSR parameter values
    ///
    pub(crate) params:  AdsrParamArrayRT,

    /// the current output of the envelope
    ///
    output:        f32,

    /// the current phase of the envelope
    ///
    phase:         f32,

    /// the sustain level of the envelope
    ///
    sustain:       f32,

    /// the current scale stage of the envelope
    ///
    scalestage:    f32,

    /// the current idle count of the envelope
    ///
    idlecount:     i32,

    /// the current state of the envelope
    ///
    envstate:      AdsrState,

    /// internal state variables used in the
    /// analog envelope processing
    _v_c1:         f32,
    _v_c1_delayed: f32,
    _discharge:    f32,

    /// a handle to a time unit object
    ///
    time_unit:     TimeUnitHandle,

    /// a handle to a tables object
    ///
    tables:        TablesHandle,

    /// a handle to a sample rate object
    ///
    srunit:        SampleRateHandle,

    /// indicates whether the envelope is enabled
    ///
    enabled:       bool
}

impl AdsrEnvelope {

    pub fn new( 
        time_unit: TimeUnitHandle,
        tables:    TablesHandle,
        srunit:    SampleRateHandle) -> Result<Self,SurgeError> {

        let mut x = Self {
            time_unit:     time_unit,
            tables:        tables,
            srunit:        srunit,
            params:        AdsrParam::new_runtime(),
            output:        0.0,
            phase:         0.0,
            sustain:       0.0,
            scalestage:    0.0,
            idlecount:     0,
            envstate:      AdsrState::Idle,
            _v_c1:         0.0,
            _v_c1_delayed: 0.0,
            _discharge:    0.0,
            enabled:       true
        };

        x.init()?;

        Ok(x)
    }

    pub fn get_samplerate(&self) -> f32 {
        self.srunit.samplerate()
    }

    pub fn load_v_c1(&self) -> __m128 {
        unsafe { _mm_load_ss(&self._v_c1) }
    }

    pub fn load_v_c1_delayed(&self) -> __m128 {
        unsafe { _mm_load_ss(&self._v_c1_delayed) }
    }

    pub fn load_discharge(&self) -> __m128 {
        unsafe { _mm_load_ss(&self._discharge) }
    }

    pub fn v_c1_ref_mut<'a>(&'a mut self) -> &'a mut f32 {
        &mut self._v_c1
    }

    pub fn v_c1_delayed_ref_mut<'a>(&'a mut self) -> &'a mut f32 {
        &mut self._v_c1_delayed
    }

    pub fn discharge_ref_mut<'a>(&'a mut self) -> &'a mut f32 {
        &mut self._discharge
    }

    pub fn output_ref_mut<'a>(&'a mut self) -> &'a mut f32 {
        &mut self.output
    }

    pub fn tsyncratio(&self, x: AdsrParam) -> f32 {

        match self.params[x].get_temposync() {
            true => self.time_unit.temposyncratio(),
            false => 1.0,
        }
    }

    /// The attack phase of an ADSR envelope is the first phase and
    /// represents the increase in amplitude over time from
    /// 0 to the maximum value. 
    ///
    /// This line calculates the rate of the attack using the
    /// `envelope_rate_linear` method from the `tables` field of the
    /// `AdsrEnvelope` struct, passing in the `lc_a` parameter. It then
    /// multiplies this rate by `tsyncratio![self, Attack]`, which is a macro
    /// that retrieves the current sync ratio for the attack phase. 
    ///
    pub fn attack_rate(&self) -> f32 {

        let lc_a = self.get_attack_parameter();

        self.tables.envelope_rate_linear(lc_a) * 
            self.tsyncratio(AdsrParam::Attack)
    }

    /// computes the decay rate from the user-set `Decay` parameter, and scales it by a time-sync
    /// ratio. 
    ///
    pub fn decay_rate(&self) -> f32 {

        let decay = self.get_decay_parameter();

        self.tables.envelope_rate_linear(decay) * self.tsyncratio(AdsrParam::Decay)
    }

    pub fn release_rate(&self) -> f32 {

        let release = self.get_release_parameter();

        self.tables.envelope_rate_linear(release) * self.tsyncratio(AdsrParam::Release)
    }

    pub fn uberrelease_rate(&self) -> f32 {
        self.tables.envelope_rate_linear(-6.5)
    }

    pub fn increment_phase(&mut self, increment: f32) {
        self.phase += increment;
    }

    pub fn phase(&self) -> f32 {
        self.phase
    }

    pub fn phase_gte_one(&self) -> bool {
        self.phase >= 1.0
    }

    pub fn set_sustain(&mut self, x: f32) {
        self.sustain = x;
    }

    pub fn release_shape(&self) -> i32 {
        pvali![self.params[AdsrParam::ReleaseShape]]
    }

    pub fn set_phase(&mut self, p: f32) {
        self.phase = p;
    }

    pub fn clear_phase(&mut self) {
        self.set_phase(0.0);
    }

    pub fn decrement_phase(&mut self, x: f32) {
        self.phase -= x;
    }

    pub fn phase_is_negative(&self) -> bool {
        self.phase < 0.0
    }

    pub fn scale_output(&mut self, x: f32) {
        self.output *= x;
    }

    pub fn set_output(&mut self, x: f32) {
        self.output = x;
    }

    pub fn clear_output(&mut self) {
        self.set_output(0.0);
    }

    pub fn set_idlecount(&mut self, x: i32) {
        self.idlecount = x;
    }

    pub fn clear_idlecount(&mut self) {
        self.set_idlecount(0);
    }

    pub fn increment_idlecount(&mut self) {
        self.idlecount += 1;
    }

    pub fn scalestage(&mut self) -> f32 {
        self.scalestage
    }

    pub fn set_scalestage(&mut self, x: f32) {
        self.scalestage = x;
    }

    pub fn reset_analog_state_machine(&mut self) {
        self._v_c1         = 0.0;
        self._v_c1_delayed = 0.0;
        self._discharge    = 0.0;
    }

    pub fn set_envstate(&mut self, x: AdsrState) {
        self.envstate = x;
    }

    pub fn envstate_is_before_release(&self) -> bool {
        self.envstate < AdsrState::Release
    }

    pub fn get_attack_parameter(&self) -> f32 {
        pvalf![self.params[AdsrParam::Attack]]
    }

    pub fn get_attack_parameter_minimum(&self) -> f32 {
        pvalminf![self.params[AdsrParam::Attack]]
    }

    pub fn get_sustain_parameter(&self) -> f32 {
        pvalf![self.params[AdsrParam::Sustain]]
    }

    pub fn state_machine_is_attack_or_decay(&self) -> bool {
        (self.envstate == AdsrState::Attack) || 
            (self.envstate == AdsrState::Decay)
    }

    pub fn state_machine_is_uberrelease(&self) -> bool {
        self.envstate == AdsrState::UberRelease
    }

    pub fn discharged(&self) -> bool {
        self._discharge == 0.0 
    }

    pub fn capacitor_voltage_is_below_silence_threshold(&self) -> bool {
        const SILENCE_THRESHOLD: f32 = 1e-6;
        self._v_c1 < SILENCE_THRESHOLD
    }
}

impl CheckEnabled for AdsrEnvelope {

    /// returns whether the ADSR envelope is
    /// currently enabled or not.
    ///
    fn enabled(&self) -> bool {
        self.enabled
    }
}

impl Enable for AdsrEnvelope {

    /// sets the enabled state of the ADSR
    /// envelope to `v`.
    ///
    fn enable(&mut self, v: bool) {
        self.enabled = v;
    }
}

impl GetModulationSourceType for AdsrEnvelope {

    /// returns the type of modulation source, in
    /// this case `ModSrcType::Adsr`.
    ///
    fn modulation_source_type(&self) -> ModSrcType
    {
        ModSrcType::Adsr
    }
}

impl GetModulationSourceOutput for AdsrEnvelope {

    /// returns the current output value of the
    /// ADSR envelope
    ///
    fn get_output(&self) -> f64 {
        todo!();
    }

    /// returns the current output value of the
    /// ADSR envelope, scaled to a range between
    /// 0 and 1
    ///
    fn get_output01(&self) -> f64 {
        todo!();
    }
}

impl SetModulationSourceOutput for AdsrEnvelope {

    /// sets the output value of the ADSR envelope
    /// to `x`, casted to a `f32`.
    ///
    fn set_output(&mut self, x: f64) {
        self.output = x as f32;
    }
}

impl CheckIsIdle for AdsrEnvelope {

    /// `AdsrEnvelope::is_idle`: This function
    /// returns `true` if the envelope is
    /// currently in an "Idle" state and the
    /// `idlecount` is greater than 0, and `false`
    /// otherwise.
    ///
    fn is_idle(&self) -> bool 
    {
        self.envstate == AdsrState::Idle && self.idlecount > 0
    }
}

impl GetAdsrEnvelopeState for AdsrEnvelope {

    /// `AdsrEnvelope::get_env_state`: This
    /// function returns the current state of the
    /// envelope.
    ///
    fn get_envstate(&self) -> AdsrState { 
        self.envstate
    }
}
