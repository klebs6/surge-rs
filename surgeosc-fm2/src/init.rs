crate::ix!();

pub fn fm2_calcmd(x: f64) -> f64 
{
    x * x * x * 8.0 * PI
}

impl Init for FM2Oscillator {
    fn init(&mut self) {

        let ph:   f64 = 2.0_f64 * PI * (self.pvalf(FM2OscillatorParam::MxStartPhase) as f64);
        let amt1: f64 = self.pvalf(FM2OscillatorParam::M1Amount).into();
        let amt2: f64 = self.pvalf(FM2OscillatorParam::M2Amount).into();

        self.lastoutput = 0.0;
        self.driftlfo = 0.0;
        self.driftlfo2 = 0.0;
        self.rm1.set_phase(ph);
        self.rm2.set_phase(ph);
        self.phase = (-ph).sin() * ( fm2_calcmd(amt1) + fm2_calcmd(amt2) ) - ph;
    }
}
