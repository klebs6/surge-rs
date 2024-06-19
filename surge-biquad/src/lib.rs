#![feature(stdarch_x86_mm_shuffle)]

#[macro_use] mod imports; use imports::*;

x!{attack_release_threshold_processor}
x!{biquad}
x!{calc}
x!{cascade}
x!{coeff_apf}
x!{coeff_bp2a}
x!{coeff_bp}
x!{coeff_hp}
x!{coeff_lp2b}
x!{coeff_lph}
x!{coeff_lp}
x!{coeff_notch}
x!{coeff_peak_eq}
x!{coeff_pka}
x!{customize_buffer_size}
x!{plot_magnitude}
x!{process_mono}
x!{process_sample_nolag_noinput}
x!{process_sample_nolag}
x!{process_sample_stereo_nolag}
x!{process_sample_stereo}
x!{process_sample}
x!{process_slowlag}
x!{process_stereo}
x!{redo}
x!{set_coeffs}
