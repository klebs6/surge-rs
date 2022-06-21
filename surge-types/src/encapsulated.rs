crate::ix!();

pub type NumberOfBlocks         = usize;
pub type OutputDataPresent      = bool;
pub type Mask32                 = u32;

//without this, we end up casting everywhere between various integer types
//and the whole thing is a mess
//
//this way, we can control the internal representation of all of our integer types in this single file)
//we can write conversion functions as necessary to get from one to another losslessly
//
//Note: this hasn't been, but could be, implemented in some way
//for most primitive types across the codebase
//
encapsulate_bool![MpeEnableSwitch];
encapsulate_bool![MappingPresent];
encapsulate_bool![TuningPresent];
encapsulate_bool![IsPopulated];
encapsulate_bool![ShouldKeepPlaying];

encapsulate_integral![NumVoices,     u32];
encapsulate_integral![ZoomFactor,    i32];
encapsulate_integral![PatchDataSize, u32];

encapsulate_integral![WaveTableID,         usize];
encapsulate_integral![WaveTableCategoryID, usize];

encapsulate_float![PitchBendRange,   f32];
encapsulate_float![PitchBendValue,   f32];

//helps enforce structure in the 
//presence of parseable data
encapsulate_string![TuningData];
encapsulate_string![MappingData];
encapsulate_string![SclFileName];
encapsulate_string![KbmFileName];
encapsulate_string![WaveTableFilename];
encapsulate_string![WaveTableDataFilename];
encapsulate_string![WaveTableWavFilename];

