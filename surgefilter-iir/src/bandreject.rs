crate::ix!();

pub struct BandRejectFilter {
    pub tuner: TunerHandle,
    pub subtype: FilterSubType,

}

impl FilterProcessQuad for BandRejectFilter {

    #[inline] fn process_quad(
        &self, 
        qfu: &mut QuadFilterUnitState, 
        input: __m128) -> __m128 
    {
        crate::iir_12_b_quad(qfu, input)
    }
}

impl CoeffMake for BandRejectFilter {

    fn coeff_make(&self, mut freq: f32, reso: f32) -> [f32; N_COEFFMAKER_COEFFS] {

        let one_m_reso   = 1.0 - reso as f64;
        let one_m_reso_2 = one_m_reso * one_m_reso;

        boundfreq(&mut freq);

        let q2inv: f64 = {

            if self.subtype == FilterSubType::Rough {

                (1.00 
                 - 0.99 * limit_range(
                     (1.0 - one_m_reso_2) as f32, 
                     0.0, 
                     1.0
                 )
                ) as f64

            } else {

                (2.5 
                 - 2.49 * limit_range(
                     (1.0 - one_m_reso_2) as f32, 
                     0.0, 
                     1.0
                 )
                ) as f64
            }
        };

        let (cosi, sinu) = self.tuner.note_to_omega::<f64,true>(freq as f64) ;

        let alpha: f64 = sinu * q2inv;

        let coeffs = FilterCoeffs::bandreject(alpha, cosi);

        coeffs.to_normalized_lattice( 1.0, 0.005 )
    }
}

