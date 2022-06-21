crate::ix!();

#[derive(Debug)]
pub struct AllpassFilter<const N: usize> {
    buffer: [f64; N],
    a:      f64,
    wpos:   usize,
}

impl<const N: usize> Default for AllpassFilter<N> {

    fn default() -> Self {
        Self {
            wpos:      0,
            a:       0.3,
            buffer: [0.0; N],
        }
    }
}

impl<const N: usize> AllpassFilter<N> {

    fn update_wpos(&mut self) {
        self.wpos = (self.wpos + 1) % N;
    }

    pub fn process(&mut self, x: f64) -> f64 {

        self.update_wpos();

        let y: f64 = self.buffer[self.wpos];

        self.buffer[self.wpos] = y * -self.a + x;

        y + self.buffer[self.wpos] * self.a
    }

    pub fn set_a(&mut self, a: f64) {
        self.a = a;
    }
}

#[test] fn test_allpass() {

    use crate::*;

    /*
       | From Wikipedia:
       |
       | An all-pass filter is a signal processing filter
       | that passes all frequencies equally in gain, but
       | changes the phase relationship among various
       | frequencies. Most types of filter reduce the
       | amplitude (i.e. the magnitude) of the signal
       | applied to it for some values of frequency,
       | whereas the all-pass filter allows all
       | frequencies through without changes in level
       */

    //question: does our allpass filter do this?
    let mut allpass = AllpassFilter::<8>::default();
    let mut signal  = Signal::white_noise(1024);

    let power_freq_tolerance = 0.01;
    let initial_power_freq   = signal.average_power_freq_domain();

    println!("initial power (freq): {:?}", initial_power_freq);

    for i in 0..10 {

        println!("iter {}", i);

        signal = Signal {
            data: signal.data.map(|x| allpass.process(*x))
        };

        let power_freq  = signal.average_power_freq_domain();

        println!("allpassed power (freq): {:?}", power_freq);

        assert!((initial_power_freq - power_freq).abs() < power_freq_tolerance);
    }
}
