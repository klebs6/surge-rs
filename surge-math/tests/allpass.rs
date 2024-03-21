// goal: test that we pass all frequencies equally in gain
//
use surge_math::*;

#[test] fn test_allpass() {

    use crate::*;

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
