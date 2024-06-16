crate::ix!();

impl Initialize for FMOscillator {

    fn init(&mut self) {

        self.phase      = 0.0;
        self.lastoutput = 0.0;
        self.driftlfo   = 0.0;
        self.driftlfo2  = 0.0;

        self.am.set_phase(0.0);
        self.rm1.set_phase(0.0);
        self.rm2.set_phase(0.0);
    }
}
