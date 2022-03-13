ix!();

use crate::*;

fn get_loop_len(spec: &hound::WavSpecSurge) -> Result<(bool,i32),WaveTableBuildError> {

    let has_clm:  bool = spec.clm.is_some();
    let has_srge: bool = spec.srge.is_some();
    let has_smpl: bool = spec.smpl.is_some();
    let has_cue:  bool = spec.cue.is_some();
    let _has_srgo: bool = spec.srgo.is_some();

    let loop_data: bool = has_smpl || has_clm || has_srge;

    let loop_len: i32 = match (has_clm, has_cue, has_srge, has_smpl) {
        (true, _, _, _) => spec.clm.unwrap().len,
        (_, true, _, _) => spec.cue.unwrap().len,
        (_, _, true, _) => spec.srge.unwrap().len,
        (_, _, _, true) => spec.smpl.unwrap().len,
        _ => -1,
    };

    if loop_len == 0 {
        return Err(WaveTableBuildError::DoesNotUnderstandFile);
    }

    Ok((loop_data,loop_len))
}

impl TryFrom<WaveTableWavFilename> for WaveTable {

    type Error = WaveTableBuildError;

    fn try_from(filename: WaveTableWavFilename) -> Result<Self,Self::Error> {

        let mut reader = match hound::WavReader::open(&filename.0) {
            Ok(r) => r,
            Err(e) => panic!("unable to open file: {:?}, error: {:?}", filename.0,e),
        };

        let spec            = reader.surge_spec();
        let audio_format    = spec.specx.unwrap().spec.sample_format;
        let bits_per_sample = spec.specx.unwrap().spec.bits_per_sample;

        let datasamples = match audio_format {
            hound::SampleFormat::Float => reader.samples::<f32>().count(),
            hound::SampleFormat::Int   => reader.samples::<i16>().count(),
        };

        if reader.seek(0).is_err() {
            return Err(WaveTableBuildError::CouldNotSeekToStart);
        }

        println!("surge_spec: {:?}, datasamples: {:?}", spec, datasamples);

        let (loop_data, loop_len) = get_loop_len(&spec)?;

        let loop_count: usize = datasamples / (loop_len as usize);

        let mut flags = WaveTableFlags::IS_SAMPLE;
        let mut dim   = WaveTableDim {
            mipmap_levels: 0,
            num_tables:    0,
            table_len:     0,
        };

        let mut sh: i32 = 0;

        if loop_data {

            flags = WaveTableFlags::CLEARED;

            sh = match loop_len {
                4096 => 12,
                2048 => 11,
                1024 => 10,
                512  => 9,
                256  => 8,
                128  => 7,
                64   => 6,
                32   => 5,
                16   => 4,
                8    => 3,
                4    => 2,
                2    => 1,
                _    => { flags = WaveTableFlags::IS_SAMPLE; 0 }
            }
        }

        if loop_len != -1 && ( sh == 0 || (loop_count as i32) < 3 ) {
            return Err(WaveTableBuildError::FileContainsInsufficientNumberOfFrames {
                filename:             filename.0,
                provided_loops:       loop_count as i32,
                provided_loop_length: loop_len,
            });
        }

        dim.table_len = 1 << sh;

        let sample_length: i32 = std::cmp::min(
            datasamples as i32, 
            (MAX_WAVETABLE_SIZE * MAX_SUBTABLES) as i32
        );

        dim.num_tables = std::cmp::min(
            MAX_SUBTABLES, 
            (sample_length >> sh) as usize
        );

        if flags.intersects(WaveTableFlags::IS_SAMPLE) {

            let has_srgo: bool = spec.srgo.is_some();

            //TODO: we don't always get this chunk -- what gives?
            let srge_len  = match spec.srge {
                Some(x) => x.len,
                _ => 0,
            };

            let mut window_size = 1024;

            if has_srgo {
                window_size = srge_len;
            }

            while window_size * 4 > sample_length && window_size > 8 {
                window_size /= 2;
            }

            dim.table_len = window_size as usize;
            dim.num_tables = ( sample_length / window_size ) as usize;
        }

        dim.mipmap_levels = required_mipmap_levels(dim.table_len);

        assert!(dim.mipmap_levels != 0);

        let header = WaveTableHeader {
            dim,
            flags 
        };

        let channels: u16 = 1;

        assert!(channels == spec.specx.unwrap().spec.channels);

        match (audio_format, bits_per_sample, channels) {
            (hound::SampleFormat::Int, 16, 1) => {
                //WAVE_FORMAT_PCM
                //assert!(header.dim.table_len * header.dim.num_tables * 2 <= datasz);

            },
            (hound::SampleFormat::Float, 32, 1) => {
                //WAVE_FORMAT_IEEE_FLOAT
                //assert!(header.dim.table_len * header.dim.num_tables * 4 <= datasz);
            },
            _ => {
                return Err(WaveTableBuildError::FileIsUnsupportedDataFormat {
                    filename:                  filename.0,
                    provided_bits_per_sample:  bits_per_sample as i32,
                    num_channels:              channels as i32,
                    format_name:               match audio_format {
                        hound::SampleFormat::Float => "Float".to_string(),
                        hound::SampleFormat::Int   => "Int".to_string(),
                    },
                });
            },
        }

        match audio_format {
            hound::SampleFormat::Float => {
                let samples: Vec<f32> = reader.samples().map(|r| r.unwrap()).collect();
                Ok(WaveTable::IeeeFloat(WaveTableBase::<f32>::new(header,samples)))
            },
            hound::SampleFormat::Int => {
                let samples: Vec<i16> = reader.samples().map(|r| r.unwrap()).collect();
                Ok(WaveTable::Pcm(WaveTableBase::<i16>::new(header,samples)))
            },
        }
    }
}
