ix!();

use crate::SurgeSynthesizer;

impl SurgeSynthesizer<'sr,'plugin_layer,'synth_out> {

    #[inline] pub fn get_split_key(&self) -> u8 {
        pvali![self.active_patch.params[PatchParam::SplitKey]].try_into().unwrap()
    }

    #[inline] pub fn get_scene_active(&self) -> i32 {
        pvali![self.active_patch.params[PatchParam::SceneActive]]
    }

    /// For split/dual
    /// MIDI Channel 1 plays the split/dual
    /// MIDI Channel 2 plays A
    /// MIDI Channel 3 plays B
    pub fn calculate_channel_mask(&mut self, channel: u8, key: u8) -> u8 {

        //A voice is routed to scene n if channelmask & n. 
        //So "1" means scene A, 
        //"2" means scene B and 
        //"3" (= 2 | 1 ) = both.
        let mut channelmask: u8 = channel;

        let scene_mode   = self.get_scene_mode();
        let split_key    = self.get_split_key();
        let scene_active = self.get_scene_active();

        let _patch = &mut self.active_patch;

        if (channel == 0) 
            || (channel > 2) 
            || self.mpe_unit.enabled().0
            || ( scene_mode == SceneMode::ChannelSplit )
        {
            channelmask = match scene_mode 
            {
                SceneMode::Single => match scene_active 
                {
                    1 => 2,
                    _ => 1,
                },

                SceneMode::Dual => 3,

                SceneMode::KeySplit => match key < split_key
                {
                    true  => 1,
                    false => 2,
                },
                SceneMode::ChannelSplit => match ( channel - 1 ) < ( split_key / 8 ) 
                {
                    true  => 1,
                    false => 2,
                },
            }
        }
        else if scene_mode == SceneMode::Single
        {
            channelmask = match scene_active {
                1 => 2,
                _ => 1,
            };
        }

        channelmask
    }
}
