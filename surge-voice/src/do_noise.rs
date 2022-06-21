crate::ix!();

impl SurgeVoice {

    pub fn process_noise(&mut self,runtime: &mut OscillatorRuntime) {

        let noise_colour = runtime.noise_colour;
        let is_wide      = runtime.is_wide;

        for i in (0..BLOCK_SIZE_OS).step_by(2) {

            unsafe {

                *runtime.tblock_l.buf.as_mut_ptr().add(i) = correlated_noise_o2mk2(
                    self.noisegen_l[0], 
                    self.noisegen_l[1], 
                    noise_colour
                );

                *runtime.tblock_l.buf.as_mut_ptr().add(i + 1) = *runtime.tblock_l.buf.as_mut_ptr().add(i);

                if is_wide {
                    *runtime.tblock_r.buf.as_mut_ptr().add(i) = correlated_noise_o2mk2(
                        self.noisegen_r[0], 
                        self.noisegen_r[1], 
                        noise_colour
                    );
                    *runtime.tblock_r.buf.as_mut_ptr().add(i + 1) = 
                        *runtime.tblock_r.buf.as_mut_ptr().add(i);
                }
            }
        }

        unsafe {
            match is_wide {
                true => { 
                    self.osclevels[LagEntry::Noise].multiply_2_blocks(
                        runtime.tblock_l.buf.as_mut_ptr(), 
                        runtime.tblock_r.buf.as_mut_ptr(), 
                        BLOCK_SIZE_OS_QUAD);
                },
                false => {
                    self.osclevels[LagEntry::Noise].multiply_block(
                        runtime.tblock_l.buf.as_mut_ptr(), 
                        BLOCK_SIZE_OS_QUAD);
                },
            }
        }

        if self.route[5] < 2 { 
            accumulate_block(
                runtime.tblock_l.buf.as_mut_ptr(),  
                self.output[0].as_mut_ptr(), 
                BLOCK_SIZE_OS_QUAD); 
        }

        if self.route[5] > 0 { 
            accumulate_block(
                runtime.tblock_r.buf.as_mut_ptr(), 
                self.output[1].as_mut_ptr(), 
                BLOCK_SIZE_OS_QUAD); 
        }
    }
}
