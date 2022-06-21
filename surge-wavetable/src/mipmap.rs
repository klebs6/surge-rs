crate::ix!();

const MIPMAP_FILTER_SIZE:  usize = 64;
const MIPMAP_FILTER_ID_OF: usize = (MIPMAP_FILTER_SIZE - 1) >> 1;

pub trait HasMipmapFilter<T: WaveTableData> {
    const HRFILTER: [T; MIPMAP_FILTER_SIZE];
}

impl HasMipmapFilter<i16> for i16 {
    const HRFILTER: [i16; MIPMAP_FILTER_SIZE] = [
        1     , 33    , -8    , -48   , 31    , 72    , -74   , -92   , 
        143   , 95    , -240  , -66   , 364   , -14   , -505  , 168   , 
        642   , -416  , -748  , 779   , 782   , -1279 , -687  , 1951  , 
        375   , -2874 , 331   , 4293  , -1957 , -7315 , 7773  , 31275 , 
        31275 , 7773  , -7315 , -1957 , 4293  , 331   , -2874 , 375   , 
        1951  , -687  , -1279 , 782   , 779   , -748  , -416  , 642   , 
        168   , -505  , -14   , 364   , -66   , -240  , 95    , 143   , 
        -92   , -74   , 72    , 31    , -48   , -8    , 33    , 1
    ];
}

impl HasMipmapFilter<f32> for f32 {
    const HRFILTER: [f32; MIPMAP_FILTER_SIZE] = [
        -9.637_663e-8    , -2.216_513_6e-6   , -1.200_509_1e-6  , 1.796_276_4e-5   , 
        1.773_084_5e-5   , -5.898_886_6e-5   , -8.980_041_5e-5  , 0.000_123_391_02 , 
        0.000_296_451_68 , -0.000_157_318_35 , -0.000_746_503_5 , 1.204_636_7e-18  , 
        0.001_525_280_3  , 0.000_660_553_5   , -0.002_588_451_4 , -0.002_282_966_6 , 
        0.003_618_633_1  , 0.005_384_810_3   , -0.003_885_820_3 , -0.010_366_649   , 
        0.002_154_163    , 0.017_290_542     , 0.003_383_208_3  , -0.025_699_832   , 
        -0.015_368_784   , 0.034_578_65      , 0.038_758_956    , -0.042_511_478   , 
        -0.089_599_33    , 0.048_023_88      , 0.312_525_42     , 0.449_999_6      , 
        0.312_525_42     , 0.048_023_88      , -0.089_599_33    , -0.042_511_478   , 
        0.038_758_956    , 0.034_578_65      , -0.015_368_784   , -0.025_699_832   , 
        0.003_383_208_3  , 0.017_290_542     , 0.002_154_163    , -0.010_366_649   , 
        -0.003_885_820_3 , 0.005_384_810_3   , 0.003_618_633_1  , -0.002_282_966_6 , 
        -0.002_588_451_4 , 0.000_660_553_5   , 0.001_525_280_3  , 1.204_636_7e-18  , 
        -0.000_746_503_5 , -0.000_157_318_35 , 0.000_296_451_68 , 0.000_123_391_02 , 
        -8.980_041_5e-5  , -5.898_886_6e-5   , 1.773_084_5e-5   , 1.796_276_4e-5   , 
        -1.200_509_1e-6  , -2.216_513_6e-6   , -9.637_663e-8    , -1.200_509_1e-6
    ];
}

///this is likely broken, naive, and parallelizable
pub fn populate_mipmaps<T: WaveTableData + HasMipmapFilter<T>>(
    data: &mut A3d::<T>,
    header: &WaveTableHeader)
{
    let base_samples  = header.dim.table_len;
    let num_tables    = header.dim.num_tables;
    let mipmap_levels = header.dim.mipmap_levels;

    let _hrfilter = T::HRFILTER;

    println!("mipmap levels: {:?}", mipmap_levels);

    for mipmap_level in 1..mipmap_levels {

        let parent_size: usize = base_samples >> (mipmap_level - 1);
        let level_size:  usize = base_samples >> mipmap_level;

        print!("parent_size: {:?}, level_size: {:?}", parent_size, level_size);

        for table in 0..num_tables {

            for sample in 0..level_size {

                for a in 0..MIPMAP_FILTER_SIZE {

                    let mut srcsample: i32 = (((sample << 1) + a) as i32) - (MIPMAP_FILTER_ID_OF as i32);
                    let srctable:      i32 = std::cmp::max(0, (table as i32) + (srcsample / (parent_size as i32)));

                    srcsample &= (parent_size - 1) as i32;

                    if srctable < (num_tables as i32) {

                        let filter_value = T::HRFILTER[a];
                        let parent_value = data[[mipmap_level -1, srctable as usize, srcsample as usize]];

                        data[[mipmap_level,table,sample]] = filter_value.maybe_saturating_mul(parent_value);
                    }
                }
            }
        }
    }
}
