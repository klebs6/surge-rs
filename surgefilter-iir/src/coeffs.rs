crate::ix!();

#[derive(Debug,Copy,Clone)]
pub struct FilterCoeffs {
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
}

impl FilterCoeffs {

    #[inline] pub fn a0inv(&self) -> f64 { 
        1.0 / self.a0 
    }

    #[inline] pub fn bandreject(alpha: f64, cosi: f64) -> Self {
        Self {
            a0: 1.0 + alpha,
            a1: -2.0 * cosi,
            a2: 1.0 - alpha,
            b0: 1.0,
            b1: -2.0 * cosi,
            b2: 1.0,
        }
    }

    #[inline] pub fn bandpass(alpha: f64, cosi: f64, q2inv: f64) -> Self {

        let quality_factor: f64 = 0.5 / q2inv;

        Self {
            a0: 1.0 + alpha,
            a1: -2.0 * cosi,
            a2: 1.0 - alpha,
            b0: quality_factor * alpha,
            b1: 0.0,
            b2: -quality_factor * alpha,
        }
    }

    #[inline] pub fn lowpass(alpha: f64, cosi: f64) -> Self {
        Self {
            a0: 1.0 + alpha,
            a1: -2.0 * cosi,
            a2: 1.0 - alpha,
            b0: (1.0 - cosi) * 0.5,
            b1: 1.0 - cosi,
            b2: (1.0 - cosi) * 0.5,
        }
    }

    #[inline] pub fn highpass(alpha: f64, cosi: f64) -> Self {
        Self {
            a0: 1.0 + alpha,
            a1: -2.0 * cosi,
            a2: 1.0 - alpha,
            b0: (1.0 + cosi) * 0.5,
            b1: -(1.0 + cosi),
            b2: (1.0 + cosi) * 0.5,
        }
    }

    #[inline] pub fn gain(reso: f64, subtype: FilterSubType) -> f64 
    {
        let x = crate::resoscale(reso, subtype);
        match subtype {
            FilterSubType::SVF => x * 2.0,
            _                  => x,
        }
    }

    #[inline] pub fn q2inv(
        reso: f64, 
        freq: f64, 
        subtype: FilterSubType, 
        pole: PoleType) -> f64 
    {
        match pole {
            PoleType::TwoPole  => map_2pole_resonance(reso, freq, subtype),
            PoleType::FourPole => map_4pole_resonance(reso, freq, subtype),
        }
    }

    #[inline] pub fn alpha(
        sinu: f64, 
        cosi: f64, 
        q2inv: f64, 
        subtype: FilterSubType) -> f64 
    {
        let x: f64 = sinu * q2inv;

        match subtype {

            FilterSubType::SVF => x,

            _ => std::cmp::min(
                FloatOrd(x),
                FloatOrd((1.0 - cosi * cosi).sqrt() - 0.0001)
            ).0,
        }
    }
}
