crate::ix!();

impl Initialize for SineWaveOscillator {

    fn init(&mut self) {
        self.phase       = 0.0;
        self.sine.set_phase(self.phase);
        self.driftlfo1   = 0.0;
        self.driftlfo2   = 0.0;
        self.lastvalue   = 0.0;
    }
}
