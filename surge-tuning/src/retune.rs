crate::ix!();

impl RetuneToStandardTuning for SurgeTuner {

    fn retune_to_standard_tuning(&mut self) {
        self.init();
    }
}

impl RetuneToScale for SurgeTuner {

    fn retune_to_scale(&mut self, scale: &crate::Scale) -> bool {

        if !scale.is_valid() {
            return false;
        }

        self.current_scale = Align16(scale.clone());
        self.current_tuning.is_standard_tuning = false;

        let pos_pitch0: i32 = 256 + self.current_mapping.tuning_constant_note;
        let pos_scale0: i32 = 256 + self.current_mapping.middle_note;

        let scale_constant_pitch = self.scale_constant_pitch();

        let pitch_mod: f32 =  scale_constant_pitch.log2() - 1.0;

        let mut _scale_position_of_start_note:  i32 = 0;
        let mut scale_position_of_tuning_note: i32 = self.current_mapping.tuning_constant_note - self.current_mapping.middle_note;

        if self.current_mapping.count > 0 {
            scale_position_of_tuning_note = self.current_mapping.keys[scale_position_of_tuning_note as usize];
        }

        let scale_count: i32 = scale.count as i32;

        let tuning_center_pitch_offset: f32 = match scale_position_of_tuning_note {
            0 => 0.0,

            _ => {
                let mut tshift: f32 = 0.0;
                let dt: f32 = scale.tones[scale.count -1].val - 1.0;

                while scale_position_of_tuning_note < 0 
                {
                    scale_position_of_tuning_note += scale_count;
                    tshift += dt;
                }

                while scale_position_of_tuning_note > (scale_count)
                {
                    scale_position_of_tuning_note -= scale_count;
                    tshift -= dt;
                }

                match scale_position_of_tuning_note {
                     0 => -tshift,
                     _ => scale.tones[(scale_position_of_tuning_note - 1) as usize].val - 1.0 - tshift,
                }
            },
        };

        let mut pitches: [f32; 512] = [0.0; 512];

        pitches[pos_pitch0 as usize] = 1.0;

        for (idx, pitch) in pitches.iter_mut().enumerate() {

            // TODO: ScaleCenter and PitchCenter are now two different notes.
            let distance_from_pitch0: i32 = (idx as i32) - pos_pitch0;
            let distance_from_scale0: i32 = (idx as i32) - pos_scale0;

            match distance_from_pitch0 {
                0 => {

                    self.tables.table_pitch[idx] = 2.0_f64.powf( (*pitch + pitch_mod).into() );

                    if cfg![debug_scales] && idx > 296 && idx < 340
                    {
                        println!("PITCH: i={}, n={}, p={}, tp={}, fr={}",
                            idx,
                            idx - 256,
                            *pitch,
                            self.tables.table_pitch[idx],
                            self.tables.table_pitch[idx] * 8.175798915
                        );
                    }

                },
                _ => {

                    /* We used to have this which assumed 1-12
                       Now we have our note number, our distance from the 
                       center note, and the key remapping
                       int rounds = (distanceFromScale0-1) / scale.count;
                       int this_round = (distanceFromScale0-1) % scale.count;
                       */

                    let mut disable: bool = false;

                    let (mut rounds, mut this_round) = 
                        match (self.current_mapping.is_standard_mapping) || (self.current_mapping.count == 0) 
                    {
                        true => {

                            let rounds     = (distance_from_scale0 - 1) / scale_count;
                            let this_round = (distance_from_scale0 - 1) % scale_count;

                            (rounds, this_round)
                        },
                        false => {
                            /*
                             ** Now we have this situation. We are at note i so we
                             ** are m away from the center note which is distanceFromScale0
                             **
                             ** If we mod that by the mapping size we know which note we are on
                             */
                            let mut mapping_key: i32 = distance_from_scale0 % self.current_mapping.count;

                            if mapping_key < 0 {
                                mapping_key += self.current_mapping.count;
                            }

                            let cm:   i32 = self.current_mapping.keys[mapping_key as usize];
                            let mut push: i32 = 0;

                            match cm < 0 {
                                true  => { disable = true },
                                false => { push = mapping_key - cm },
                            };

                            let rounds     = (distance_from_scale0 - push - 1) / scale_count;
                            let this_round = (distance_from_scale0 - push - 1) % scale_count;

                            if cfg![debug_scales] &&  idx > 296 && idx < 340 {
                                println!("MAPPING n={}, pushes ds0={}, cmc={}, tr={}, r={}, mk={}, cm={}, push={}, dis={}, mk-p-1={}",
                                    idx - 256,
                                    distance_from_scale0,
                                    self.current_mapping.count,
                                    this_round,
                                    rounds,
                                    mapping_key,
                                    cm,
                                    push,
                                    disable,
                                    mapping_key - push - 1
                                );
                            }
                            (rounds, this_round)
                        },
                    };


                    if this_round < 0 {
                        this_round += scale_count;
                        rounds -= 1;
                    }

                    let _mul: f32 = scale.tones[scale.count-1].val.pow(rounds);

                    *pitch = match disable {
                        true  => 0.0,
                        false => { scale.tones[this_round as usize].val + (rounds as f32) * (scale.tones[scale.count - 1].val - 1.0) - tuning_center_pitch_offset },

                    };

                    let otp: f64 = self.tables.table_pitch[idx];

                    self.tables.table_pitch[idx] = 2.0_f64.powf( (*pitch + pitch_mod).into() );

                    if cfg![debug_scales] && idx > 296 && idx < 340 {
                        println!(
                            "PITCH: i={}, n={}, ds0={}, dp0={}, r={}, t={}, p={}, t={} {}, dis={}, tp={}, fr={}, otp={}, tcpo={}, diff={}",
                            idx,
                            idx - 256,
                            distance_from_scale0,
                            distance_from_pitch0,
                            rounds,
                            this_round,
                            *pitch,
                            scale.tones[this_round as usize].val,
                            scale.tones[this_round as usize],
                            disable,
                            self.tables.table_pitch[idx],
                            self.tables.table_pitch[idx] * 8.175798915,
                            otp,
                            tuning_center_pitch_offset,
                            self.tables.table_pitch[idx] - otp
                        );
                    }
                }
            };
        }

        let dsamplerate_os_inv = self.srunit.dsamplerate_os_inv();

        for idx in 0..512 {

            let table_pitch = self.tables.table_pitch[idx];

            self.tables.table_pitch_inv[idx] = 1.0 / table_pitch;

            let arg: f64 = 2.0 * PI * mind(
                0.5, 
                CONCERT_A_HZ * (table_pitch as f64) * dsamplerate_os_inv
            );

            self.tables.table_note_omega[[0,idx]] = arg.sin();
            self.tables.table_note_omega[[1,idx]] = arg.cos();
        }

        true
    }
}
