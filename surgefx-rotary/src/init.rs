crate::ix!();

impl Suspend for RotarySpeaker {

    fn suspend(&mut self) -> Result<(),SurgeError> {
        self.buffer.fill(0.0);
        self.xover.suspend()?;
        self.lowbass.suspend()?;
        self.wpos = 0;
        Ok(())
    }
}

impl Initialize for RotarySpeaker {

    fn init(&mut self) -> Result<(),SurgeError> {
        self.buffer.fill(0.0);
        self.xover.suspend()?;
        self.lowbass.suspend()?;
        self.xover.coeff_lp2b(self.xover.calc_omega(0.862496), 0.707);
        self.lowbass.coeff_lp2b(self.xover.calc_omega(-1.14), 0.707);
        self.wpos = 0;

        Ok(())
    }
}
