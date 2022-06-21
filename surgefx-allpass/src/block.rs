crate::ix!();

pub struct AllpassBlockCfg<'a> {
    lfos:  &'a [f32; ALLPASS_REVERB_NUM_BLOCKS],
    block: usize, 
    input: f32, 
    x:     &'a mut f32, 
    out_l: &'a mut f32, 
    out_r: &'a mut f32,
}

impl crate::AllpassVerb {

    pub fn do_allpass_block(&mut self, cfg: AllpassBlockCfg ) 
    {
        let lfos  = cfg.lfos;
        let block = cfg.block;
        let input = cfg.input;
        let x     = cfg.x;
        let out_l = cfg.out_l;
        let out_r = cfg.out_r;

        *x += input;

        for allpassidx in 0..ALLPASS_REVERB_NUM_ALLPASSES_PER_BLOCK {

            *x = self.allpass[[block,allpassidx]].process(*x, self.buildup.v);
        }

        *x = self.hf_damper[block].process_lowpass(*x, self.hf_damp_coefficient.v);
        *x = self.lf_damper[block].process_highpass(*x, self.lf_damp_coefficient.v);

        let modulation: i32 = (
            self.modulation.v * 
            lfos[block] * 
            ALLPASS_REVERB_DELAY_SUBSAMPLE_RANGE as f32
        ) as i32; 

        let tap_out_l: f32;
        let tap_out_r: f32;

        (*x, tap_out_l, tap_out_r) = 
            self.delay[block].process(*x, 
                self.tap_time_l[block], 
                self.tap_time_r[block], 
                modulation);

        *out_l += tap_out_l * self.tap_gain_l[block];
        *out_r += tap_out_r * self.tap_gain_r[block];

        *x *= self.decay_multiply.v;

    }

    pub fn do_process_block<const N: usize>(&mut self, 
        k: usize, 
        pdt: i32, 
        wet: &mut WetBlock,
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        let mut input: f32 = (data_l[k] + data_r[k]) * 0.5;

        input = self.predelay.process( input, pdt );

        input = self.input_allpass[0].process(input, self.diffusion.v);
        input = self.input_allpass[1].process(input, self.diffusion.v);
        input = self.input_allpass[2].process(input, self.diffusion.v);
        input = self.input_allpass[3].process(input, self.diffusion.v);

        let mut x: f32 = self.state;

        let mut out_l: f32 = 0.0;
        let mut out_r: f32 = 0.0;

        let lfos: [f32; ALLPASS_REVERB_NUM_BLOCKS] = [
            self.lfo.r  as f32,
            self.lfo.i  as f32,
            -self.lfo.r as f32,
            -self.lfo.i as f32
        ];

        for block in 0..ALLPASS_REVERB_NUM_BLOCKS {
            self.do_allpass_block(
                AllpassBlockCfg {
                    lfos: &lfos, 
                    block, 
                    input, 
                    x:     &mut x, 
                    out_l: &mut out_l, 
                    out_r: &mut out_r
                }
            );
        }

        wet.l[k] = out_l;
        wet.r[k] = out_r;

        self.state = x;

        self.decay_multiply.process();
        self.diffusion.process();
        self.buildup.process();
        self.hf_damp_coefficient.process();
        self.lfo.process();
        self.modulation.process();
    }
}
