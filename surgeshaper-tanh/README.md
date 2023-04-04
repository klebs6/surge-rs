## TanhShaper

The `TanhShaper` is an implementation of a soft
clipping algorithm using the hyperbolic tangent
function. This shaper can be used to add
saturation and warmth to an audio signal, similar
to a tube amplifier. The amount of shaping can be
controlled by adjusting the `shape` parameter.

### Mathematical analysis

The hyperbolic tangent function is defined as:

```
tanh(x) = (exp(x) - exp(-x)) / (exp(x) + exp(-x))
```

When `x` is small, `tanh(x)` is approximately
equal to `x`, and as `x` becomes larger, `tanh(x)`
approaches 1. This non-linear response curve makes
it suitable for use as a shaper function.

The `TanhShaper` takes an input signal and applies
the hyperbolic tangent function to it, which has
the effect of softly clipping the signal. The
degree of soft clipping is controlled by the
`shape` parameter, which scales the input signal.

### Usage

The `TanhShaper` can be used in any application
where saturation or warmth is desired, such as in
guitar amplifiers, synthesizers, or mixing
consoles. It can be used in conjunction with other
shaping algorithms to create complex and unique
sounds.

### Caveats

One potential issue with using a soft clipping
algorithm like `TanhShaper` is that it can
introduce additional harmonic distortion to the
signal. This can result in a loss of clarity or
fidelity, depending on the
application. Additionally, care must be taken when
adjusting the `shape` parameter to avoid
introducing unwanted artifacts or distortion.

Overall, the `TanhShaper` is a powerful tool for
shaping audio signals and can be a valuable
addition to any audio processing toolbox.

94> could you teach me about the hyperbolic
tangent function?

Sure! The hyperbolic tangent function, usually
denoted as `tanh(x)`, is a mathematical function
that maps any real number `x` to a value between
-1 and 1. The function is defined as:

```
tanh(x) = (e^x - e^-x) / (e^x + e^-x)
```

where `e` is the mathematical constant
approximately equal to 2.71828.

The graph of the hyperbolic tangent function is
similar to the graph of the standard tangent
function, but with a vertical scaling of 1 and
a horizontal scaling of 2. The function is
symmetric about the origin, meaning that 
`tanh(-x) = -tanh(x)` for all `x`.

The hyperbolic tangent function is often used as
an activation function in artificial neural
networks because it is differentiable and has
a range between -1 and 1. It is also commonly used
in signal processing, particularly in audio and
image processing, for its ability to compress and
expand dynamic range.

95> could you show me some python code
illustrating the effect on the input signal?

Certainly, here is an example of using the `tanh`
function to shape an input signal:

```python
import numpy as np
import matplotlib.pyplot as plt

# Create a sine wave input signal
sample_rate = 44100
duration = 1.0
freq = 440.0
t = np.linspace(0, duration, int(sample_rate * duration), endpoint=False)
signal = np.sin(2 * np.pi * freq * t)

# Apply tanh shaping with a shape factor of 0.5
shape_factor = 0.5
shaped_signal = np.tanh(signal * shape_factor)

# Plot the input and output signals
fig, ax = plt.subplots(2, 1)
ax[0].plot(t, signal)
ax[0].set_title('Input Signal')
ax[0].set_xlabel('Time (s)')
ax[0].set_ylabel('Amplitude')
ax[1].plot(t, shaped_signal)
ax[1].set_title('Output Signal')
ax[1].set_xlabel('Time (s)')
ax[1].set_ylabel('Amplitude')
plt.show()
```

In this example, we create a 1 second sine wave
input signal with a frequency of 440 Hz sampled at
a rate of 44100 Hz. We then apply the `tanh`
function with a shape factor of 0.5 to the input
signal to create the shaped output signal. The
resulting input and output signals are then
plotted for visualization.
