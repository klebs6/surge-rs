ix!();

use crate::*;

/**
  | post-silence we can implement with
  | zeros for out of bounds access or we can
  | use a "padded-dimension" abstraction
  |
  */
pub fn populate_base_mipmap_level<T: WaveTableData>(
    data: &mut A3d::<T>,
    header: &WaveTableHeader,
    samples: Vec::<T>)
{
    let num_tables = header.dim.num_tables;
    let table_len  = header.dim.table_len;
    let this_mipmap_level = 0;

    assert!(
        samples.len() == 
        (num_tables * table_len)
    );

    let mut cur_table        = 0;
    let mut cur_table_sample = 0;

    /**
      |tk -- this quick implementation of mine may
      |be totally incorrect, as far as I can tell,
      |there are several factors which need to be
      |considered. It seems to me that ndarray
      |takes care of many of them, in
      |fact. a higher level of abstraction will
      |help greatly.
      |
      |The idea of "append_silence" as well as zero
      |padding for interpolation are noted, though
      |I am not sure we need them at this level.
      |IMO it is better to bring all of the data
      |into the program, and arrange it minimally
      |in light of our desired mental model. This,
      |to me, is simply a 3D ndarray, indexed by
      |(miplevel, table, sample)
      |
      |Any client code should access interpolated
      |values via an API which hides the underlying
      |data representation.
      |
      |This API should function regardless of the
      |underlying extents in the three dimensions.
      */
    for sample in samples.iter() {

        data[[this_mipmap_level, cur_table, cur_table_sample]] = *sample;

        cur_table_sample += 1;

        if cur_table_sample == table_len {
            cur_table_sample = 0;
            cur_table += 1;
        }
    }
}

impl<T: WaveTableData + HasMipmapFilter<T>> WaveTableBase<T> {

    pub fn data_3d_from_samples( header: WaveTableHeader, samples: Vec::<T>) -> A3d::<T> {
        let mut data = A3d::<T>::zeros((
                header.dim.mipmap_levels, 
                header.dim.num_tables,
                header.dim.table_len));
        populate_base_mipmap_level(&mut data, &header, samples);
        populate_mipmaps(&mut data, &header);
        data
    }

    pub fn new( header: WaveTableHeader, samples: Vec::<T>) -> Self 
    {
        assert!(header.dim.num_tables >= 3,
            "WaveTable need at least 3 tables to work properly"
        );

        Self {
            header: header.clone(),
            data: Self::data_3d_from_samples(header,samples),
        }
    }
}
