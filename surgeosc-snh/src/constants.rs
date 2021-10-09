/// const float integrator_hpf = 0.99999999f;
/// const float integrator_hpf = 0.9992144f;  // 44.1 kHz
/// const float integrator_hpf = 0.9964f;     // 44.1 kHz
/// const float integrator_hpf = 0.9982f;     // 44.1 kHz  Magic Moog freq
pub const SNH_INTEGRATOR_HPF:            f32 = 0.999;

/// 290 samples to fall 50% (British) (is probably a 2-pole HPF)
/// 202 samples (American)
/// pow(ln(0.5)/(samplerate/50hz)
pub const SNH_HPF_CYCLE_LOSS:            f32 = 0.995;
