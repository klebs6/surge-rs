ix!();

use crate::SurgeScene;

impl SurgeScene {

    pub fn release_old_notes_if_previous_polymode_was_latch<P: Param + ?Sized>(
        &mut self, 
        param: &mut ParamRT<P>,
        oldvali: i32)
    {
        let oldpolymode    = PolyMode::try_from(oldvali as usize).unwrap();
        let parampolymode  = PolyMode::try_from(pvali![param] as usize).unwrap();

        if oldpolymode == PolyMode::LatchMonophonic 
            && parampolymode == PolyMode::LatchMonophonic 
        {
            self.release_if_latched = true;
            self.release_anyway     = true;
        }
    }

    //note, this function had much more logic in C++
    //what is written here is something like a placeholder until
    //we find out which bugs exist
    pub fn set_parameter01<P: Param + ?Sized>(
        &mut self, 
        param: &mut ParamRT<P>, 
        value: f32, 
        force_integer: bool) -> bool 
    {
        let mut need_refresh: bool = false;

        let oldvali = pvali![param];
        let oldvalb = pvalb![param];

        param.set_value_f01(value,force_integer);

        if param.affect_other_parameters() {
            need_refresh = true;
        }

        let ctrltype = param.control_type();

        match ctrltype {
            ControlType::SceneMode  => self.set_release_if_latched(true),
            ControlType::PolyMode   => self.release_old_notes_if_previous_polymode_was_latch(param,oldvali),
            ControlType::WaveshapeType | ControlType::BoolMute | ControlType::BoolFM | ControlType::FbConfig | ControlType::FilterSubType => {
                self.switch_toggled_queued = true;
            },
            ControlType::BoolRelativeSwitch => {
                self.handle_boolrelative_switching(param, oldvalb);
                need_refresh = true;
            },
            ControlType::BoolLinkSwitch | ControlType::EnvelopeMode => {
                // See github issue #160
                need_refresh = true;
            },
            ControlType::BoolSolo => {
                self.handle_boolsolo(param);
                self.switch_toggled_queued = true;
                need_refresh = true; 
            },
            _ => { /* no-op */ },
        }

        need_refresh
    }

    #[inline] pub fn maybe_switch_toggled(&mut self) {
        if self.switch_toggled_queued 
        {
            self.switch_toggled();
            self.switch_toggled_queued = false;
        }
    }
}
