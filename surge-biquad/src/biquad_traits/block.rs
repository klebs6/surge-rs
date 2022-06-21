pub trait ProcessBlockMono {

    /**
      |# Safety
      |
      |data_l and data_r must each point to
      |BLOCK_SIZE valid contiguous data elements
      */
    unsafe fn process_block_mono(
        &mut self, 
        data: *mut f32, 
        out: Option<*mut f32>);
}

pub trait ProcessBlockStereo {

    /**
      |# Safety
      |
      |data_l and data_r must each point to
      |BLOCK_SIZE valid contiguous data elements
      */
    unsafe fn process_block_stereo(
        &mut self, 
        data_l: *mut f32, 
        data_r: *mut f32, 
        out: Option<(*mut f32, *mut f32)>
    );
}

pub trait ProcessBlockSlowlag {

    /**
      |# Safety
      |
      |data_l and data_r must each point to
      |BLOCK_SIZE valid contiguous data elements
      */
    unsafe fn process_block_slowlag(
        &mut self, 
        data_l: *mut f32, 
        data_r: *mut f32);
}

