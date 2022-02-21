ix!();

use crate::{
    Conditioner,
    ConditionerParam,
};

#[derive(Debug)]
pub struct ConditionerProcessCfg {
    pub attack_rate:  f32,
    pub release_rate: f32,
    pub threshold:    f32,
    pub balance:      f32,
    pub width:        f32,
    pub gain:         f32,
    pub am:           f32,
    pub rm:           f32,
    pub attack:       f32,
    pub release:      f32,
    pub a:            f32,
    pub pregain:      f32,
}
 
impl ConditionerProcessCfg {
    pub fn new(xxx: &Conditioner) -> Self {

        let attack_rate:  f32 = xxx.pvalf(ConditionerParam::AttackRate);
        let release_rate: f32 = xxx.pvalf(ConditionerParam::ReleaseRate);
        let threshold:    f32 = xxx.pvalf(ConditionerParam::Threshold);
        let balance:      f32 = xxx.pvalf(ConditionerParam::Balance);
        let width:        f32 = xxx.pvalf(ConditionerParam::Width);
        let gain:         f32 = xxx.pvalf(ConditionerParam::Gain);
        let am:           f32 = 1.0 + 0.9 * attack_rate;
        let rm:           f32 = 1.0 + 0.9 * release_rate;
        let attack:       f32 = 0.001 * am * am;
        let release:      f32 = 0.0001 * rm * rm;
        let a:            f32 = xxx.srunit.vu_falloff();

        let mut x = Self {
            attack_rate,
            release_rate,
            threshold,
            balance,
            width,
            gain,
            am,
            rm,
            attack,
            release,
            a,
            pregain: 0.0,
        };
        x.pregain = xxx.tables.db_to_linear(- threshold);
        x
    }
}
