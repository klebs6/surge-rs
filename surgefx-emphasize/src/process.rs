ix!();

use crate::{
    Emphasize,
};

impl Process for Emphasize<'sr> {

    fn process<const N: usize>(&mut self, 
        _data_l: &mut [f32; N], 
        _data_r: &mut [f32; N]) 
    {
        todo!();
    }
}


