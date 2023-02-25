/// 14> could you describe more about how filter
/// coefficients are chosen, more generally? but
/// please be specific as to how this applies to
/// the audio domain and which equations may
/// produce better results
/// 
/// Certainly! In general, the process of choosing
/// filter coefficients for a digital filter
/// involves selecting a specific filter type
/// (such as a low-pass, high-pass, or band-pass
/// filter) and then determining the optimal
/// values for the coefficients of that filter to
/// achieve the desired frequency response.
/// 
/// In the audio domain, the selection of filter
/// coefficients depends on the specific
/// application and the desired frequency
/// response. For example, in equalization (EQ)
/// processing, the goal is to adjust the
/// frequency balance of an audio signal. This is
/// typically done by using a set of filter
/// coefficients to selectively boost or cut
/// certain frequency bands. The choice of filter
/// coefficients will depend on the specific
/// equalization curve being used, which will be
/// determined by the mixing engineer or producer
/// based on their desired tonal balance.
/// 
/// When designing filters for audio processing,
/// there are several mathematical techniques that
/// can be used to generate filter
/// coefficients. One common approach is to use
/// windowed-sinc filter design, which involves
/// selecting a specific window function (such as
/// a Hamming or Kaiser window) and using it to
/// generate a finite-duration impulse response
/// (FIR) filter. The optimal filter coefficients
/// are then determined by performing a Fourier
/// transform of the desired frequency response
/// and using it to calculate the filter kernel.
/// 
/// Another approach is to use IIR (infinite
/// impulse response) filter design techniques,
/// such as Butterworth, Chebyshev, or Elliptic
/// filter designs. These methods produce filters
/// with a smoother frequency response than FIR
/// filters, but at the cost of increased
/// complexity and potential instability. The
/// choice of filter design will depend on the
/// specific requirements of the application,
/// including the desired frequency response,
/// phase response, and passband and stopband
/// ripple.
/// 
/// Ultimately, the choice of filter coefficients
/// for audio processing depends on the specific
/// application and the desired frequency
/// response. There are many techniques and
/// equations available for designing filters, and
/// the best approach will depend on the specific
/// requirements of the application, including the
/// desired frequency response, phase response,
/// and passband and stopband ripple.

#![feature(stdarch)]

#[macro_use] mod imports; use imports::*;

x![coeff];
x![halfratefilter];
x![loadstore];
x![process_block];
x![process_block_d2];
x![process_block_u2];
x![softer];
x![steep];
x![scratch];
