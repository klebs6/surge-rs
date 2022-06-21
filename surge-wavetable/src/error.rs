crate::ix!();

#[derive(Debug,Clone)]
pub enum WaveTableBuildError {
    FileContainsNoValidRIFFHeaderChunk {
        filename:                  String,
    },
    FileNotStandardRIFFWaveFile {
        filename:                  String,
        header_riff:               [char; 4],
        header_wave:               [char; 4],
    },
    FileIsUnsupportedDataFormat {
        filename:                  String,
        provided_bits_per_sample:  i32,
        num_channels:              i32,
        format_name:               String,
    },
    FileContainsBadMetadata {
        filename:                  String,
    },
    FileContainsInsufficientNumberOfFrames {
        filename:                  String,
        provided_loops:            i32,
        provided_loop_length:      i32,
    },
    UnknownWaveFileParseError {
        filename:                  String,
    },
    UnableToOpenFile {
        filename:                  String,
    },
    DoesNotUnderstandFile,
    CouldNotSeekToStart,
    TODO,
}
