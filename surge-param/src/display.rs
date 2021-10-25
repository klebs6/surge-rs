ix!();

use crate::{ParameterDisplay,ParamRT,Param};

#[inline] pub fn get_notename( i_value: i32 ) -> String
{
    let octave: i32 = (i_value / 12) - 2;
    let notenames: [&'static str;  12] =  [
        "C ", "C#", "D ", "D#", "E ", "F ", "F#", "G ", "G#", "A ", "A#", "B "
    ];
    let idx: usize = (i_value % 12).try_into().unwrap();
    format!("{}{}", notenames[idx], octave)
}

//TODO find a better way
pub fn tempo_sync_notation_value(val: f32) -> String {

    let (mut integral, mut fractional) = split_float(val);

    if fractional < 0.0 {
        fractional += 1.0;
        integral -= 1.0;
    }

    let mut d: f32;
    let mut q: f32;
    let mut nn: String;
    let t:  String;

    match val >= 1.0 {
        true => {

            q  = 2.0_f32.powf( val - 1.0 );
            nn = "whole".into();

            match q {
                _ if q >= 3.0 => {
                    return format!("{:0.2} whole notes", q);
                },

                _ if q >= 2.0 => {
                    nn = "double whole".into();
                    q /= 2.0;
                },
                _ => {},
            }

            t = match q {
                _ if q < 1.3 => {
                    "note".into()
                },
                _ if q < 1.4 => {
                    if nn.eq("whole") { 
                        nn = "1/2".into(); 
                    }
                    "triplet".into()
                },
                _ => { "dotted".into() },
            };

        },
        false => {

            d = 2.0_f32.powf( - (integral - 2.0) );
            q = 2.0_f32.powf( fractional + 1.0 );

            t = match q {
                _ if q < 1.3 => { "note".into() },
                _ if q < 1.4 => { d /= 2.0; "triplet".into() },
                _            => { "dotted".into() },
            };

            nn = match (d - 1.0).abs() < f32::EPSILON {
                true  => "whole".into(),
                false => format!("1/{:0.5}", d),
            };
        }
    }
    return format!("{} {}", nn, t)
}

/*
  |the reason these are commented is because they
  |create circular references between
  |crates. Param should not need to know the
  |implementation details of its users.  We can
  |refactor this, or simply get rid of it.
  |
  |I think it makes sense to create printing
  |functions associated with each relevant
  |ControlType, and dispatch to these for the
  |param implementors that need them.
  |
  |To me, it seems better to implement a display
  |function (if desired) for a param implementor
  |at the point of implementation
  */
impl<P: Param> ParameterDisplay for ParamRT<P> {

    #[allow(unused_variables)]
    fn get_display(&self, external: bool, ef: f32) -> String {
        todo!();

        /*
        let mut result: String = "".into();

        if self.control_type() == ControlType::Nil {
            result = "-".into();
            return result;
        }

        match (self.val, self.min_value(), self.max_value()) {
            (PData::Float(_f), PData::Float(min), PData::Float(max))  => {
                let f = match external {
                    true  => ef * (max - min) + min,
                    false => _f,
                };
                match self.control_type() {
                    ControlType::PortaTime 
                        | ControlType::EnvTime 
                        | ControlType::EnvTimeLfoDecay 
                        | ControlType::ReverbTime 
                        | ControlType::ReverbPreDelayTime 
                        | ControlType::DelayModTime => 
                        {
                            match (self.control_type() 
                                == ControlType::EnvTimeLfoDecay) 
                                && (f == max) 
                                {
                                    true => result = format!("forever"),
                                    false => {
                                        if self.temposync {
                                            result = format!("{}", tempo_sync_notation_value(f));
                                        }else if f == min {
                                            result = "0.000 s".into();
                                        } else {
                                            result = format!("{:0.3} s", 2.0_f32.powf(_f));
                                        }
                                    }
                                } 
                        },
                    ControlType::LfoRate => {
                        match self.temposync {
                            true => {
                                result = tempo_sync_notation_value(-f);
                            },
                            false => {
                                result = format!("{:0.3} Hz", 2.0_f32.powf(f));
                            },
                        }
                    },
                    ControlType::Amplitude => {
                        match f == 0.0 {
                            true  => { result = format!("-inf dB")}, 
                            false => { result = format!("{:0.2} dB", amp_to_db(f))}, 
                        }
                    },
                    ControlType::Decibel 
                        | ControlType::DecibelAttenuation 
                        | ControlType::DecibelAttenuationLarge 
                        | ControlType::DecibelFMDepth 
                        | ControlType::DecibelNarrow 
                        | ControlType::DecibelExtraNarrow => 
                        {
                            result = format!("{:0.2} dB", f);
                        },
                        ControlType::DecibelExtendable 
                            | ControlType::DecibelNarrowExtendable => 
                            {
                                result = format!("{:0.2} dB", self.get_extended(f));
                            },
                        ControlType::Bandwidth => {
                            result = format!("{:0.2} octaves", f);
                        },
                        ControlType::FreqShift => {
                            result = format!("{:0.2} Hz", self.get_extended(f));
                        },
                        ControlType::Percent 
                            | ControlType::PercentBidirectional 
                            | ControlType::CountedSetPercent => 
                            {
                                result = format!("{:0.1} {}", f * 100.0, '%');
                            },
                        ControlType::OscSpread => 
                        {
                            match self.absolute {
                                true  => { result = format!("{:0.1} Hz", 16.0 * self.get_extended(f) ); },
                                false => { result = format!("{:0.1} cents", self.get_extended(f) * 100.0 ); },
                            }
                        },
                        ControlType::Detuning => 
                        {
                            result = format!("{:0.1} cents", f * 100.0 );
                        },
                        ControlType::StereoWidth => 
                        {
                            result = format!("{:0.1}º", f );
                        },
                        ControlType::FreqHpf 
                            | ControlType::FreqAudible 
                            | ControlType::FreqVocoderLow 
                            | ControlType::FreqVocoderHigh => 
                            {
                                result = format!("{:0.3} Hz", CONCERT_A_HZ * 2.0_f64.powf((f as f64) / 12.0));
                            },
                        ControlType::FreqMod => 
                        {
                            result = format!("{:0.2} semitones", f);
                        },
                        ControlType::PitchSemi7BP => 
                        {
                            result = format!("{:0.2}", self.get_extended(f) );
                        },
                        ControlType::PitchSemi7BPAbsolutable => 
                        {
                            match self.absolute {
                                true  => { result = format!("{:0.1} Hz", 10.0 * self.get_extended(f)); },
                                false => { result = format!("{:0.2}",    self.get_extended(f)); },
                            }
                        },
                        ControlType::FlangerPitch => 
                        {
                            result = format!("{:0.2}", f);
                        },
                        _ => 
                        {
                            result = format!("{:0.2}", f);
                        },
                }
            },
            (PData::Int(mut i), PData::Int(min), PData::Int(max)) => {
                if external {
                    i = ((1.0 / 0.99) * (ef - 0.005) * ((max - min) as f32) + 0.5) as i32 + min;
                }
                match self.control_type() {
                    ControlType::OscType => { 
                        let ty = OscillatorType::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::Wt2Window => { 
                        let ty = WindowType::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::FxType => { 
                        let ty = EffectType::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::FxBypass => { 
                        let ty = FxBypassType::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::FilterType => { 
                        let ty = FilterType::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::WaveshapeType => { 
                        let ty = WaveshapeType::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::EnvelopeMode => { 
                        let ty = EnvelopeMode::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::FbConfig => { 
                        let ty = FilterBlockConfiguration::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::FmConfig => { 
                        let ty = FmConfiguration::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::LfoShape => { 
                        let ty = LfoShape::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::SceneMode => { 
                        let ty = SceneMode::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::PolyMode => { 
                        let ty = PolyMode::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::LfoTrigMode => { 
                        let ty = LfoMode::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::Character => { 
                        let ty = CharacterMode::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::SineOscMode => { 
                        let ty = OscillatorType::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::SineFMLegacy => { 
                        if i == 0 {
                            result = "Legacy (1.6.1.1 and earlier)".into();
                        } else {
                            result = "Consistent w. FM2/3".into();
                        }
                    },
                    ControlType::VocoderBandcount => { 
                        result = format!("{} bands", i);
                    },
                    ControlType::DistortionWaveshape => { 
                        let ty = WaveshapeType::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    },
                    ControlType::OscRoute => { 
                        let ty = OscRoute::try_from(i as u32).unwrap();
                        result = ty.to_string();
                    }
                    ControlType::FlangerMode => { 
                        let wave = FlangerWave::try_from(i as usize % FlangerWave::count()).unwrap();
                        let ty = FlangerType::try_from(i as usize / FlangerType::count()).unwrap();
                        result = format!("wave: {}, type: {}", wave, ty);
                    }
                    _ => { 
                        result = format!("{}",i);
                    }
                }
            },
            (PData::Bool(b), _, _) => {
                match external {
                    true  => result = match ef > 0.5 { true => "true".into(), false => "false".into() },
                    false => result = match        b { true => "true".into(), false => "false".into() },
                }
            },
            _ => unreachable!(),
        }
        result
            */
    }

    #[allow(unused_variables)]
    fn get_display_alt(&self, external: bool, ef: f32) -> String {
        todo!();

        /*
        let mut result: String = "".into();

        match self.control_type() {

            ControlType::FreqHpf 
                | ControlType::FreqAudible 
                | ControlType::FreqVocoderLow 
                | ControlType::FreqVocoderHigh => 
                {
                    match self.val {
                        PData::Float(f) => {
                            let mut i: i32 = (f + 0.5) as i32 + 69;// that 1/2th centers us
                            if i < 0 { 
                                i = 0;
                            }
                            result = format!("~{}", get_notename(i));
                        },
                        _ => unreachable!()
                    }
                },
            ControlType::FlangerPitch => {
                match self.val {
                    PData::Float(f) => {
                        let mut i: i32 = f as i32;
                        if i < 0 { 
                            i = 0;
                        }
                        result = format!("~{}", get_notename(i));

                    },
                    _ => unreachable!()
                }
            },
            ControlType::CountedSetPercent => {
                todo!("write this one we figure out how to factor out the CountedSetUserData");
                /*
                   if (user_data != nullptr)
                   {
                // We check when set so the reinterpret cast is safe and fast
                float f = val.f;
                CountedSetUserData* cs = reinterpret_cast<CountedSetUserData*>(user_data);
                auto count = cs->getCountedSetSize();
                auto tl = count * f;
                sprintf(txt, "%.1f / %d", tl, count);
                }
                */
            },
            _ => {},
        }
        result
            */
    }
}

