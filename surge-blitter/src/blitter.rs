crate::ix!();

#[derive(Debug)]
pub struct AbstractBlitter {
    pub oscbuffer_l:         Align16<A1d::<f32>>,
    pub oscbuffer_r:         Align16<A1d::<f32>>,
    pub dcbuffer:            Align16<A1d::<f32>>,
    pub osc_out_l:           __m128,
    pub osc_out_2l:          __m128,
    pub osc_out_r:           __m128,
    pub osc_out_2r:          __m128,
    pub integrator_hpf:      f32,
    pub pitchmult:           f32,
    pub pitchmult_inv:       f32,
    pub n_unison:            i32,
    pub bufpos:              i32,
    pub out_attenuation:     f32,
    pub out_attenuation_inv: f32,
    pub detune_bias:         f32,
    pub detune_offset:       f32,

    pub oscstate:            A1d::<f32>,
    pub syncstate:           A1d::<f32>,
    pub rate:                A1d::<f32>,
    pub driftlfo:            A1d::<f32>,
    pub driftlfo2:           A1d::<f32>,
    pub pan_l:               A1d::<f32>,
    pub pan_r:               A1d::<f32>,
    pub state:               A1d::<i32>,
}
