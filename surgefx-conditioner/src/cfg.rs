ix!();

use crate::{
    Conditioner,
    ConditionerParam,
};

#[derive(Debug)]
pub struct ConditionerProcessCfg {
    pub attack_rate_f:  f32,
    pub release_rate_f: f32,
    pub threshold_f:    f32,
    pub balance_f:      f32,
    pub width_f:        f32,
    pub gain_f:         f32,
    pub am:             f32,
    pub rm:             f32,
    pub attack:         f32,
    pub release:        f32,
    pub a:              f32,
    pub pregain:        f32,
}
 
impl ConditionerProcessCfg {
    pub fn new(xxx: &Conditioner) -> Self {

        let attack_rate_f:  f32 = xxx.pvalf(ConditionerParam::AttackRate);
        let release_rate_f: f32 = xxx.pvalf(ConditionerParam::ReleaseRate);
        let threshold_f:    f32 = xxx.pvalf(ConditionerParam::Threshold);
        let balance_f:      f32 = xxx.pvalf(ConditionerParam::Balance);
        let width_f:        f32 = xxx.pvalf(ConditionerParam::Width);
        let gain_f:         f32 = xxx.pvalf(ConditionerParam::Gain);
        let am:             f32 = 1.0 + 0.9 * attack_rate_f;
        let rm:             f32 = 1.0 + 0.9 * release_rate_f;
        let attack:         f32 = 0.001 * am * am;
        let release:        f32 = 0.0001 * rm * rm;
        let a:              f32 = xxx.srunit.vu_falloff();

        let mut x = Self {
            attack_rate_f,
            release_rate_f,
            threshold_f,
            balance_f,
            width_f,
            gain_f,
            am,
            rm,
            attack,
            release,
            a,
            pregain: 0.0,
        };
        x.pregain = xxx.tables.db_to_linear(- threshold_f);
        x
    }
}
