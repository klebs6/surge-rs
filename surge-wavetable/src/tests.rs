ix!();

use crate::{
    WaveTable,
};

#[test] fn wavetable_read() {

    let should_run_test = false;

    if should_run_test {

        //TODO: need to fix this
        let root = "path/to/resources";

        //"resources/wavetables/basic/sinetosawtoothhq.wav",
        //"resources/wavetables/basic/tri-sawhq.wav",
        //"resources/wavetables/basic/sinetosquarehq.wav",
        //"resources/wavetables/hollo/glitter.wav",
        //"resources/wavetables/hollo/lampio.wav",
        //"resources/wavetables/hollo/op_ep_body.wav",
        let files = [
            "resources/wavetables/generated/sawatc.wav",
            "resources/wavetables/generated/squareatc.wav",
            "resources/wavetables/sampled/harpsi.wav",
            "resources/wavetables/basic/sinehq.wav",
            "resources/wavetables/basic/trianglehq.wav",
            "resources/wavetables/basic/sineoctaveshq.wav",
        ];

        for file in files.iter() {
            let filename = WaveTableWavFilename(
                format!("{}/{}",root,file)
            );
            let wt = WaveTable::try_from(filename);
            println!("wt: {:?}",wt);
        }
    }
}
