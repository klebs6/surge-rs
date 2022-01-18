ix!();

use crate::SurgeSynthesizer;

impl SurgeSynthesizer<'plugin_layer,'synth_out> {

    #[inline] pub fn check_scene_modsource_enabled(&self, scene: usize, modsrc: ModSource) -> bool {
        self.active_patch.scene[scene].modsources[modsrc].as_ref().unwrap().enabled()
    }

    #[inline] pub fn get_fx_bypass_type(&self) -> FxBypassType {
        let idx = pvali![self.active_patch.params[PatchParam::FxBypass]];
        FxBypassType::try_from( idx as usize).unwrap()
    }

    #[inline] pub fn get_scene_mode(&self) -> SceneMode {
        let idx = pvali![self.active_patch.params[ PatchParam::SceneMode]];
        SceneMode::try_from( idx as usize ).unwrap()
    }
}
