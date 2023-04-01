## surge-modulation

This Rust crate, `surge-modulation`, is
a component of the larger Surge synthesizer
system. It provides a set of tools and structures
for working with modulation sources and routing
modulation between different parts of the
synthesizer.

### Modulation Sources

The `ModulationSource` trait defines a common
interface for all types of modulation sources,
such as LFOs, envelopes, and custom
controllers. Each modulation source can be enabled
or disabled, and can produce a modulation value
that can be used to modulate a variety of
parameters within the synthesizer.

Several types of modulation sources are included
in the crate, including LFOs (`voice_lfo` and
`scene_lfo`), envelopes (`attack` and `release`),
and custom controllers (`ctrl`). These modulation
sources can be used to create a wide range of
modulations, from subtle filter sweeps to complex
rhythmic patterns.

### Modulation Routing

The `ModulationRouting` structure provides a way
to route modulation sources to different
parameters within the synthesizer. Each
`ModulationRouting` consists of a source (an
instance of `ModulationSource`) and a target
parameter (e.g., a filter cutoff frequency).

Modulation routings can be enabled or disabled,
and can be bipolar or unipolar depending on the
modulation source used. The `ModulationRouting`
structure also supports per-voice modulation,
which allows different voices in a polyphonic
patch to have different modulation values.

### Controller Modulation Sources

The `ControllerModulationSource` trait provides
a way to use MIDI controllers as modulation
sources. This allows the user to use knobs,
sliders, and other MIDI controllers to modulate
parameters within the synthesizer.

### Math and Algorithms

The `surge-modulation` crate uses a variety of
mathematical concepts and algorithms to implement
its modulation sources and routing. For example,
the LFO modulation sources use sine waves to
produce modulation values, while the envelope
modulation sources use exponential curves to
produce attack and release values.

Overall, `surge-modulation` is a powerful and
flexible Rust crate for working with modulation
sources and routing modulation within
a synthesizer. Its modular design and flexible
interface make it easy to integrate with other
components of the Surge synthesizer system, as
well as with other software and hardware
synthesizers.

18> what can be a modulation source?

In a synthesizer, a modulation source is any
source of control signals that can be used to
modulate (i.e., alter) the value of a target
parameter. Modulation sources can be used to add
movement, variation, and expressiveness to
a sound, and are a key part of many synthesizer
patches and sounds.

There are many types of modulation sources that
can be used in a synthesizer, including:

1. **LFOs**: Low-frequency oscillators (LFOs)
   produce a continuous waveform at a low
   frequency (usually less than 20 Hz) that can be
   used to modulate parameters such as pitch,
   filter cutoff, and amplitude.

2. **Envelopes**: Envelopes produce a series of
   control signals that vary over time, typically
   with an attack-decay-sustain-release (ADSR)
   envelope shape. Envelopes can be used to
   modulate parameters such as filter cutoff,
   amplitude, and pitch.

3. **MIDI Controllers**: MIDI controllers, such as
   knobs, sliders, and buttons on a MIDI
   controller device, can be used as modulation
   sources to control parameters in real-time.

4. **Custom Controllers**: Custom controllers,
   such as expression pedals or touch controllers,
   can also be used as modulation sources to
   control specific parameters in a unique way.

5. **Random Generators**: Random generators
   produce random values that can be used to add
   unpredictability and variation to a sound.

6. **Audio Signals**: Audio signals, such as noise
   or other sound sources, can be used as
   modulation sources to create complex and
   dynamic sounds.

Overall, the types of modulation sources that can
be used in a synthesizer are limited only by the
imagination of the designer and the capabilities
of the synthesizer.

19> let's get imaginative -- what are some others?

Sure, here are some more imaginative examples of
modulation sources that can be used in
a synthesizer:

1. **Weather Sensors**: Sensors that measure
   weather data, such as temperature, wind speed,
   and humidity, can be used as modulation sources
   to control various parameters in
   a synthesizer. For example, wind speed could be
   used to modulate a filter cutoff frequency,
   while temperature could be used to modulate an
   oscillator's frequency.

2. **Motion Sensors**: Motion sensors, such as
   accelerometers or gyroscopes, can be used as
   modulation sources to control parameters based
   on physical movement. For example,
   a synthesizer could use a motion sensor to
   modulate a filter cutoff frequency based on the
   movement of the performer's hands.

3. **Biofeedback Sensors**: Biofeedback sensors,
   such as heart rate monitors or brainwave
   sensors, can be used as modulation sources to
   create sounds that respond to the performer's
   physiological state. For example, a synthesizer
   could use a heart rate monitor to modulate
   a filter cutoff frequency based on the
   performer's heart rate.

4. **Artificial Intelligence**: Artificial
   intelligence algorithms, such as neural
   networks or genetic algorithms, can be used as
   modulation sources to create sounds that evolve
   and change over time. For example,
   a synthesizer could use a genetic algorithm to
   generate new sounds based on the parameters of
   an existing sound.

5. **Game Controllers**: Game controllers, such as
   joysticks or gamepads, can be used as
   modulation sources to create sounds that
   respond to the movement and actions of the
   performer. For example, a synthesizer could use
   a gamepad to modulate a filter cutoff frequency
   based on the position of the joystick.

Overall, the possibilities for modulation sources
in a synthesizer are limited only by the
creativity and ingenuity of the designer.

20> can you do 5 more?

Sure, here are some more ideas:

1. **Brain-Computer Interfaces**: Brain-computer
   interfaces (BCIs) can be used as modulation
   sources to control parameters based on the
   performer's brainwaves. For example,
   a synthesizer could use a BCI to modulate
   a filter cutoff frequency based on the
   performer's level of concentration or
   relaxation.

2. **Geolocation**: Geolocation data, such as GPS
   coordinates or Wi-Fi signal strength, can be
   used as modulation sources to control
   parameters based on the performer's physical
   location. For example, a synthesizer could use
   GPS coordinates to modulate a filter cutoff
   frequency based on the performer's distance
   from a particular location.

3. **Machine Learning**: Machine learning
   algorithms can be used as modulation sources to
   create sounds that respond to patterns or
   trends in data. For example, a synthesizer
   could use a machine learning algorithm to
   modulate a filter cutoff frequency based on the
   patterns in a stock market index.

4. **Speech Recognition**: Speech recognition
   software can be used as a modulation source to
   control parameters based on the performer's
   spoken words. For example, a synthesizer could
   use speech recognition to modulate a filter
   cutoff frequency based on the performer's voice
   pitch or volume.

5. **Physically-Modeled Systems**:
   Physically-modeled systems, such as models of
   physical objects or systems, can be used as
   modulation sources to create sounds that
   respond to the physical properties of the
   modeled system. For example, a synthesizer
   could use a physically-modeled model of
   a guitar string to modulate a filter cutoff
   frequency based on the tension of the string.

21> thanks -- what can you tell me about routing
in the context of modulation

Routing in the context of modulation refers to the
process of directing a modulation source to
a target parameter in order to modulate that
parameter. Modulation routing is an essential part
of synthesizer design, as it allows modulation
sources to be applied to a wide range of
parameters within the synthesizer, creating
complex and dynamic sounds.

There are many ways to route modulation in
a synthesizer, depending on the specific design
and architecture of the synthesizer. Here are
a few common methods:

1. **Hardwired Routing**: Hardwired routing is
   a method of modulation routing where
   a modulation source is connected directly to
   a target parameter using physical wiring. This
   method is common in analog synthesizers, where
   modulation sources such as LFOs and envelopes
   are often hardwired to specific parameters.

2. **Patchable Routing**: Patchable routing is
   a method of modulation routing where a user can
   manually connect modulation sources to target
   parameters using patch cables. This method is
   common in modular synthesizers, where the user
   can create complex routing configurations by
   patching cables between different modules.

3. **Menu-Driven Routing**: Menu-driven routing is
   a method of modulation routing where a user
   selects a modulation source and a target
   parameter from a menu or interface. This method
   is common in digital synthesizers and software
   synthesizers, where the user can select from
   a wide range of modulation sources and target
   parameters.

4. **Matrix Routing**: Matrix routing is a method
   of modulation routing where a user can select
   a modulation source and a target parameter from
   a matrix or grid interface. This method is
   common in digital and software synthesizers,
   where the user can create complex modulation
   routing configurations by selecting from
   a large number of modulation sources and target
   parameters.

5. **Automated Routing**: Automated routing is
   a method of modulation routing where the
   synthesizer automatically routes modulation
   sources to target parameters based on
   pre-defined rules or algorithms. This method is
   common in some software synthesizers, where the
   user can specify modulation routing using
   algorithms such as genetic algorithms or neural
   networks.

Overall, modulation routing is an essential part
of synthesizer design, allowing modulation sources
to be applied to a wide range of parameters within
the synthesizer to create complex and dynamic
sounds. The specific method of modulation routing
used in a synthesizer depends on the specific
design and architecture of the synthesizer, as
well as the needs and preferences of the user.

22> could you show an example of how to integrate
a modulation routing system into a lightweight
synthesizer architecture?

Sure, here's an example of how to integrate
a modulation routing system into a lightweight
synthesizer architecture:

Let's assume we have a lightweight synthesizer
architecture that includes a single oscillator,
a filter, and an amplifier. We want to add
modulation to this architecture by routing an LFO
to the filter cutoff frequency and the amplifier
level. Here's how we can do it:

1. **Create an LFO Module**: We start by creating
   an LFO module that produces a continuous
   waveform at a low frequency. We can define this
   module as a struct that implements
   a `ModulationSource` trait, which allows us to
   enable or disable the LFO and produce
   a modulation value.

```rust
pub struct LFO {
    enabled: bool,
    frequency: f32,
    phase: f32,
    amplitude: f32,
}

impl ModulationSource for LFO {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn get_modulation_value(&mut self) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let value = self.amplitude * (self.phase * 2.0 * std::f32::consts::PI).sin();
        self.phase += self.frequency / SAMPLE_RATE as f32;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
        value
    }
}
```

2. **Create a Modulation Routing Module**: Next,
   we create a modulation routing module that
   takes a modulation source and a target
   parameter and applies the modulation source to
   the target parameter. We can define this module
   as a struct that includes a reference to the
   modulation source and a reference to the target
   parameter.

```rust
pub struct ModulationRouting {
    source: Box<dyn ModulationSource>,
    target: TargetParameter,
}

impl ModulationRouting {
    fn new(source: Box<dyn ModulationSource>, target: TargetParameter) -> Self {
        Self { source, target }
    }

    fn process_block(&mut self, block: &mut [f32]) {
        let value = self.source.get_modulation_value();
        for sample in block.iter_mut() {
            *sample = self.target.process_sample(*sample, value);
        }
    }
}
```

3. **Create Target Parameter Modules**: Finally,
   we create modules for the target parameters
   that we want to modulate, in this case the
   filter cutoff frequency and the amplifier
   level. These modules should implement
   a `TargetParameter` trait that includes
   a `process_sample()` method that takes a sample
   and a modulation value and returns a processed
   sample.

```rust
pub trait TargetParameter {
    fn process_sample(&mut self, sample: f32, modulation_value: f32) -> f32;
}

pub struct Filter {
    cutoff_frequency: f32,
}

impl TargetParameter for Filter {
    fn process_sample(&mut self, sample: f32, modulation_value: f32) -> f32 {
        let frequency = self.cutoff_frequency + modulation_value;
        // Apply filter processing to the sample using the frequency value
        // ...
        processed_sample
    }
}

pub struct Amplifier {
    level: f32,
}

impl TargetParameter for Amplifier {
    fn process_sample(&mut self, sample: f32, modulation_value: f32) -> f32 {
        let level = self.level + modulation_value;
        // Apply amplifier processing to the sample using the level value
        // ...
        processed_sample
    }
}
```

4. **Create Synthesizer Module**: Finally, we
   create a synthesizer module that connects the
   LFO module to the modulation

23> thanks -- please continue from #4

4. **Create Synthesizer Module**: Finally, we
   create a synthesizer module that connects the
   LFO module to the modulation routing modules,
   which in turn are connected to the target
   parameter modules. This module should include
   a method to set up the modulation routing, and
   a `process_block()` method that processes
   a block of audio samples by passing the block
   through the oscillator, filter, and amplifier
   modules, and then through the modulation
   routing modules.

```rust
pub struct Synthesizer {
    oscillator: Oscillator,
    filter: Filter,
    amplifier: Amplifier,
    lfo: LFO,
    filter_modulation: ModulationRouting,
    amplifier_modulation: ModulationRouting,
}

impl Synthesizer {
    fn new() -> Self {
        let lfo = LFO {
            enabled: true,
            frequency: 2.0,
            phase: 0.0,
            amplitude: 50.0,
        };
        let filter = Filter { cutoff_frequency: 1000.0 };
        let amplifier = Amplifier { level: 0.5 };

        let filter_modulation = ModulationRouting::new(
            Box::new(lfo.clone()),
            Box::new(filter),
        );
        let amplifier_modulation = ModulationRouting::new(
            Box::new(lfo),
            Box::new(amplifier),
        );

        Self {
            oscillator: Oscillator::new(),
            filter,
            amplifier,
            lfo,
            filter_modulation,
            amplifier_modulation,
        }
    }

    fn set_modulation_routing(&mut self) {
        self.filter_modulation.source.set_enabled(true);
        self.filter_modulation.target.set_bipolar(true);

        self.amplifier_modulation.source.set_enabled(true);
        self.amplifier_modulation.target.set_bipolar(true);
    }

    fn process_block(&mut self, block: &mut [f32]) {
        self.oscillator.process_block(block);
        self.filter.process_block(block);
        self.amplifier.process_block(block);

        self.filter_modulation.process_block(block);
        self.amplifier_modulation.process_block(block);
    }
}
```

With this setup, we have created a lightweight
synthesizer architecture that includes modulation
routing. We can use this architecture to create
complex and dynamic sounds by routing modulation
sources to a wide range of target parameters
within the synthesizer.
