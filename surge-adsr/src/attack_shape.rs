crate::ix!();

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum AdsrEnvelopeAttackShape {
    Zero,
    One,
    Two,
}

impl GetAttackShape for AdsrEnvelope {

    type AttackShape = AdsrEnvelopeAttackShape;

    fn get_attack_shape(&self) -> Self::AttackShape {
        match pvali![self.params[AdsrParam::AttackShape]] {
            0 => AdsrEnvelopeAttackShape::Zero,
            1 => AdsrEnvelopeAttackShape::One,
            2 => AdsrEnvelopeAttackShape::Two,
            _ => panic!("for the AdsrEnvelope, the AttackShape paramter should be a member of the set: {0,1,2}"),
        }
    }
}
