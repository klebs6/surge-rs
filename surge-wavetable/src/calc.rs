ix!();

use crate::{
    WAVETABLE_SAMPLE_PADDING,
};

enhanced_enum![
    WaveTableFileFormat { 
        Wt, 
        Wav 
    }
];


enhanced_enum![
    PatchFileFormat { 
        Fxp 
    }
];

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
}

#[test] fn extension_from_filename() {
    assert_eq!(get_extension_from_filename("abc.gz"), Some("gz"));
}

pub fn get_wavetable_format(filename: &str) -> Option<WaveTableFileFormat> {
    let extension = get_extension_from_filename(filename).unwrap();
    match extension {
        "wt"  => Some(WaveTableFileFormat::Wt),
        "wav" => Some(WaveTableFileFormat::Wav),
        _ => None,
    }
}

pub fn get_patch_format(filename: &str) -> Option<PatchFileFormat> {
    let extension = get_extension_from_filename(filename).unwrap();
    match extension {
        "fxp"  => Some(PatchFileFormat::Fxp),
        _ => None,
    }
}

/// Calculate the worst-case scenario of the needed samples 
/// for a specific wavetable and see if it fits
pub fn required_wt_size(
    mut table_size:  usize, 
    mut table_count: usize) -> usize 
{
    let mut size: usize = 0;

    table_count += WAVETABLE_SAMPLE_PADDING; // for sample padding. 

    while table_size > 0 {

        size += table_count * (table_size + FIR_OFFSET_I16 + FIR_IPOL_I16_N);

        table_size >>= 1;
    }

    size
}

pub fn get_wt_index(
    wave_idx:  usize, 
    wave_size: usize, 
    num_waves: usize, 
    mip_map:   usize, 
    padding:  Padding) -> usize 
{
    let num_units_padding: usize = 
        match padding {
            NO_PADDING => 0,
            Padding::Units(x) => x,
        };

   let mut index: usize = wave_idx * 
       ((wave_size >> mip_map) + num_units_padding);

   let offset: usize = num_waves * wave_size;

   for i in (0_usize..mip_map).step_by(1) 
   {
      index += offset >> i;
      index += num_units_padding * num_waves;
   }

   assert!(
       (index + wave_size - 1) 
       < MAX_WAVETABLE_SAMPLES
   );

   index as usize
}

