/*
crate::ix!();

#[cfg(test)]
mod test {

    use crate::{BiquadFilter, FilterType, ProcessBlockStereo};

    const BLOCK_SIZE: usize = 128;
    const SAMPLE_RATE: f32 = 48000.0;
    const CUTOFF_FREQ: f32 = 1000.0;
    const Q: f32 = 1.0;

    #[test]
    fn test_biquad_filter_stereo() {
        let mut filter = BiquadFilter::new(SAMPLE_RATE, CUTOFF_FREQ, Q, FilterType::LowPass);

        let mut input_l = [0.0; BLOCK_SIZE];
        let mut input_r = [0.0; BLOCK_SIZE];
        for i in 0..BLOCK_SIZE {
            input_l[i] = (i as f32) / (BLOCK_SIZE as f32);
            input_r[i] = (BLOCK_SIZE as f32 - i as f32) / (BLOCK_SIZE as f32);
        }

        let mut output_l = [0.0; BLOCK_SIZE];
        let mut output_r = [0.0; BLOCK_SIZE];
        let out = (&mut output_l[..], &mut output_r[..]);

        unsafe {
            filter.process_block_stereo(
                input_l.as_mut_ptr(),
                input_r.as_mut_ptr(),
                Some(out),
            );
        }

        assert!(!output_l.iter().all(|&x| x == 0.0));
        assert!(!output_r.iter().all(|&x| x == 0.0));
    }
}

*/
