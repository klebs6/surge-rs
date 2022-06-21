crate::ix!();

pub struct VocoderBlockCfg {
    pub max_level:  __m128,
    pub rate:       __m128,
    pub rate_m1:    __m128,
    pub gate_level: __m128,
}

impl VocoderBlockCfg {
    pub fn new(xxx: &Vocoder) -> Self {

        let f_rate:      f32 = xxx.pvalf(VocoderParam::Rate);
        let f_gatelevel: f32 = xxx.pvalf(VocoderParam::GateLevel);

        let env_f_rate:  f32 = 0.001 * 2.0_f32.powf(4.0 * f_rate);
        let gate:        f32 = xxx.tables.db_to_linear(f_gatelevel + xxx.gain());

        Self{ 
            max_level:    v_load1(6.0),
            rate:         v_load1(env_f_rate),
            rate_m1:      v_load1(1.0 - env_f_rate),
            gate_level:   v_load1(gate * gate),
        }
    }
}
