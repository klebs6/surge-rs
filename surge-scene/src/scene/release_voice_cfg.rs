crate::ix!();

///goes downward
pub type KeyRange = Rev<Range<usize>>;

#[derive(Debug)]
pub struct ReleaseCfg {
    pub do_release:   bool,
    pub velocity:     u8,
    pub key:          u8,
    pub channel:      u8,
    pub keyrange:     KeyRange,
}

/** It used to be, that we did this to calculate our keyrange:

  let scenemode = self.get_scene_mode();
  let splitkey = pvali![self.active_patch.params[PatchParam::SplitKey]];
  match (scenemode, voice.state.scene_id) {
  (SceneMode::KeySplit, 0) => {
  hikey = (splitkey - 1) as u8;

  },
  (SceneMode::KeySplit, _) => {
  lowkey = splitkey as u8;

  },
  (_, _) => { /* no-op */ },
}
*/
impl SurgeScene {

    pub fn get_release_voice_cfg(&self, 
        channel: u8, 
        key: u8, 
        velocity: u8,
        keyrange: &Option<KeyRange>) -> ReleaseCfg 
    {
        let keyrange = match keyrange {
            Some(keyrange) => keyrange.clone(),
            None => {
                let lowkey: u8 = 0;
                let hikey:  u8 = 127;
                (lowkey as usize..hikey as usize + 1).rev()
            },
        };

        ReleaseCfg {
            do_release:   false,
            velocity,
            key,
            channel,
            keyrange,
        }
    }
}
