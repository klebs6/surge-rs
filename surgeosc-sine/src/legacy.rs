crate::ix!();

impl SineWaveOscillator {

    pub fn process_block_legacy(&mut self, 
        pitch: f32, 
        drift: f32, 
        stereo: bool, 
        fm: bool, 
        fmdepth: f32) 
    {
        let wf_mode: i32 = self.pvali(SineWaveOscillatorParam::Shape);

        if fm {
            self.driftlfo1 = drift_noise(self.driftlfo2);

            let omega: f64 = mind(
                PI, 
                self.tuner.pitch2omega(
                    (pitch + 
                    drift * self.driftlfo1) as f64)
            );

            self.fm_depth.new_value(fmdepth as f64);

            for k in 0..BLOCK_SIZE_OS {

                let master_osc: f64 = unsafe { 
                    *self.master_osc.add(k) as f64 
                };

                self.out.l[k] = Self::value_from_sin_and_cos(
                    self.phase.sin() as f32, 
                    self.phase.cos() as f32,
                    wf_mode,
                );

                self.phase += 
                    omega + master_osc * self.fm_depth.v;

                self.fm_depth.process();

            }

        } else {
            self.driftlfo1 = drift_noise(self.driftlfo2);

            self.sine.set_rate(
                mind(PI, 
                    self.tuner.pitch2omega(
                        (pitch + 
                        drift * self.driftlfo1) as f64)
                )
            );

            for k in 0..BLOCK_SIZE_OS {

                self.sine.process();

                let svalue: f32 = self.sine.r as f32;
                let cvalue: f32 = self.sine.i as f32;

                self.out.l[k] = 
                    Self::value_from_sin_and_cos( 
                        svalue, 
                        cvalue,
                        wf_mode);
            }
        }

        if stereo {
            self.out.dup_channel_to_stereo(
                StereoChannel::Left);
        }
    }
}
