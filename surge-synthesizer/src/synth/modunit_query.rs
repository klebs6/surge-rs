ix!();

use crate::SurgeSynthesizer;

//-------------------------------------------------------------------------------------------------
impl SurgeSynthesizer<'plugin_layer> {

    pub fn is_valid_modulation<P: Param + ?Sized>(&mut self, p: &ParamRT<P>, modsource: ModSource) -> bool 
    {
        let failure_conditions: Vec<bool> = vec![
            (modsource == ModSource::Original),

            (!p.modulateable()),

            (p.value_type() != (ValType::VtFloat)),

            (!p.per_voice_processing && !modsource.can_modulate_monophonic_target()),

            ((p.control_group() == ControlGroup::Lfo) && !modsource.is_scenelevel()),

            ((p.control_group() == ControlGroup::Lfo) && !modsource.can_modulate_modulators()),
        ];

        !failure_conditions.iter().any(|x| x == &true)
    }
}
