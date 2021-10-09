ix!();

use crate::{
    SurgeSynthesizer,
    traits::PluginLayerIF,
};

impl SurgeSynthesizer<'sr,'plugin_layer,'synth_out> {

    pub fn reset_patch(&mut self) {
        self.active_patch.init();
    }

    pub fn process_threadunsafe_operations(&mut self) {
        if !self.audio_processing_active {
            // if the audio processing is inactive, patchloading should occur anyway
            if self.patchid_queue.unwrap() >= 0 {

                //self.load_patch(self.patchid_queue);

                if cfg![target_lv2] {
                    self.plugin_layer.patch_changed();
                }

                self.patchid_queue = None;
            }

            if self.fx_unit.load_fx_needed {

                self.fx_unit.load_fx(false, false);

            }
        }
    }

    #[allow(unreachable_code)]
    pub fn handle_patchid_queue(&mut self) {

        //let mut mfade: f32 = 1.0;

        let old_masterfade = self.synth_out.masterfade();
        let new_masterfade = maxf(0.0, old_masterfade - 0.025);

        self.synth_out.set_masterfade(new_masterfade);

        //mfade = new_masterfade * new_masterfade;

        if new_masterfade < 0.0001 {

            // spawn patch-loading thread
            self.controller.halt_engine = true;

             todo!();
            /*
            thread::spawn(|| {
                //assert!(set_current_thread_priority(ThreadPriority::Min).is_ok());
                self.background_load_patch();
            });
            */

             unsafe {
                 clear_block::<BLOCK_SIZE_QUAD>(self.synth_out.out_l());
                 clear_block::<BLOCK_SIZE_QUAD>(self.synth_out.out_r());
             }
        }
    }
}
