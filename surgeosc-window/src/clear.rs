crate::ix!();

impl WindowOscillator {

    #[inline] pub fn clear(&mut self) {
        self.pos.fill(0);
        self.sub_pos.fill(0);
        self.ratio.fill(0);
        self.table.fill(0);
        self.formant_mul.fill(0);
        self.dispatch_delay.fill(0);
        self.gain.fill(0);
        self.drift_lfo.fill(0.0);
        self.fm_ratio.fill(0);
    }
}
