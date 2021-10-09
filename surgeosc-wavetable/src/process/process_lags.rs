ix!();

use crate::{
    WTOscillator,
};

impl WTOscillator<'sr> {

    #[inline] pub fn process_lag(&mut self) { 
        self.update_lagvals::<false>();
        self.l_shape.process();
        self.l_vskew.process();
        self.l_hskew.process();
        self.l_clip.process();
    }
}
