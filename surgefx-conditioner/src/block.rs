ix!();

use crate::{
    Conditioner,
    ConditionerProcessCfg,
    CONDITIONER_LOOKAHEAD,
};

impl Conditioner<'sr> {

    pub fn do_conditioner_block<const N: usize>(&mut self, 
        k: usize, 
        cfg: &ConditionerProcessCfg, 
        data_l: &mut [f32; N],
        data_r: &mut [f32; N]) 
    {
        let bufpos: usize = self.bufpos.try_into().unwrap();

        let d_l: f32 = self.delayed[[0,bufpos]];
        let d_r: f32 = self.delayed[[1,bufpos]];

        let la: f32 = self.get_lookahead();

        self.update_filtered_lamax(cfg.attack, cfg.release, la);

        self.update_gain();

        self.update_delayed(bufpos, k, data_l, data_r);

        self.update_lamax(bufpos, k, data_l, data_r);

        self.do_lookahead();

        data_l[k] = self.gain * d_l;
        data_r[k] = self.gain * d_r;

        self.increment_bufpos();
    }

    #[inline] fn update_delayed<const N: usize>(
        &mut self, 
        bufpos: usize, 
        k: usize, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        self.delayed[[0,bufpos]] = data_l[k];

        self.delayed[[1,bufpos]] = data_r[k];
    }

    #[inline] fn update_lamax<const N: usize>(
        &mut self, 
        bufpos: usize, 
        k: usize, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        self.lamax[bufpos] = maxf( 
            data_l[k].abs(), 
            data_r[k].abs()
        );

        self.lamax[bufpos] = 
            self.lamax[bufpos] * self.lamax[bufpos]; //RMS
    }

    #[inline] fn update_filtered_lamax(
        &mut self, 
        attack: f32, 
        release: f32, 
        la: f32) 
    {
        self.filtered_lamax = (1.0 - attack) * 
            self.filtered_lamax + attack * la;

        self.filtered_lamax2 = (1.0 - release) * 
            self.filtered_lamax2 + (release * self.filtered_lamax);

        if self.filtered_lamax > self.filtered_lamax2 
        {
            self.filtered_lamax2 = self.filtered_lamax;
        }
    }

    #[inline] fn update_gain(&mut self) {
        self.gain = rcp(self.filtered_lamax2);
    }

    #[inline] fn increment_bufpos(&mut self) {

        self.bufpos = (self.bufpos + 1) & ((CONDITIONER_LOOKAHEAD - 1) as i32);
    }
}
