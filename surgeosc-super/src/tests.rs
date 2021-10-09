ix!();

use crate::SurgeSuperOscillator;

#[test] fn smoke() {

    let srunit  = SampleRateHandle::default();
    let tables  = TablesHandle::new(&srunit);
    let tuner   = TunerHandle::new(&srunit);

    let mut osc = SurgeSuperOscillator::new(
        tuner.clone(),
        tables.clone(),
        srunit.clone()
    );

    osc.init();

    println!("osc: {:?}", osc);

    let cfg = OscillatorProcessBlockCfg {
        pitch:    432.0,
        drift:    0.0,
        stereo:   false,
        fm:       false,
        fm_depth: 0.0,
    };

    osc.process_block(cfg);
    //println!("osc: {:?}", osc);
}
