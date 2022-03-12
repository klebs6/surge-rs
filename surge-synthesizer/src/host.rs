ix!();

const BUFFER_SIZE: usize = 512;

use coreaudio::audio_unit::*;

#[allow(dead_code)]
pub struct SampleHost;

impl Host for SampleHost {

    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", 
            index, 
            value);
    }

    fn process_events(&self, events: &Events) {

        //other event types can be handled here
        for event in events.events() {
            if let Event::Midi(ev) = event {
                println!("process midi event: {:?}",ev.data);
                //self.process_midi_event(ev.data)
            }
        }
    }
}

impl SampleHost{
    pub fn run( &mut self, mut instance: PluginInstance) 
        -> Result<(), coreaudio::Error> {

        //--------------create AudioUnit for system output on OSX
        let mut audio_unit = {
            let audio_unit = AudioUnit::new(IOType::DefaultOutput)?;
            let stream_format  = audio_unit.output_stream_format()?;

            assert!(SampleFormat::F32 == stream_format.sample_format);
            println!("{:#?}", &stream_format);
            audio_unit 
        };

        type Args = render_callback::Args<data::NonInterleaved<f32>>;

        let in1: Vec<f32> = (0..BUFFER_SIZE).map(|x| x as f32).collect();

        let in2 = in1.clone();

        let mut out1 = vec![0.0; BUFFER_SIZE];
        let mut out2 = out1.clone();

        let inputs      = vec![in1.as_ptr(), in2.as_ptr()];

        let mut outputs = vec![out1.as_mut_ptr(), out2.as_mut_ptr()];

        let mut buffer =
            unsafe { AudioBuffer::from_raw(2, 2, 
                inputs.as_ptr(), 
                outputs.as_mut_ptr(), 
                BUFFER_SIZE) 
            };

        audio_unit.set_render_callback(move |args| {

            instance.process(&mut buffer);

            let (_inputs, outputs) = buffer.split();

            let mut iter = outputs[0].iter();

            let Args { num_frames, mut data, .. } = args;

            for i in 0..num_frames {
                let sample = iter.next().unwrap();
                for channel in data.channels_mut() {
                    channel[i] = *sample;
                }
            }

            Ok(())

        })?;

        audio_unit.start()?;

        loop {
            let dur = Duration::from_millis(300);
            thread::sleep(dur);
            if 1 == 2 {
                break;
            }
        }

        Ok(())
    }
}
