ix!();

use crate::{
    Obxd12dBCoeff,
    Obxd24dBCoeff,
    ObxdParams,
    Poles,
};

pub struct ObxdFilter<'sr> {
    pub poles:  Poles,
    pub sub:    i32,
    pub tuner:  TunerHandle<'sr>,
    pub srunit: SampleRateHandle<'sr>,
}

pub mod obxd_filter {

    use super::*;

    pub mod newton_raphson_12db {
        use super::*;

        pub struct SelfOsc {
            pub enabled_mask: __m128,
            pub on_val:       __m128,
            pub off_val:      __m128,
        }

        impl SelfOsc {
            pub unsafe fn cfb(&self) -> __m128 {
                _mm_add_ps(
                    _mm_and_ps(
                        self.enabled_mask, 
                        self.on_val), 
                    _mm_andnot_ps(
                        self.enabled_mask, 
                        self.off_val))
            }
        }
    }

    /**
      | Taylor approximation of a slightly mismatched diode pair
      |
      | return    ((f * x) + 1.0f)
      | where f = ((e * x) + 0.05)
      | where e = ((d * x) + 0.185)
      | where d = ((c * x) + 0.00920833)
      | where c = 0.0103592
      |
      */
    #[inline] pub unsafe fn diode_pair_resistance_approx(x: __m128) -> __m128 
    {
        let d = _mm_fmadd_ps(_mm_set1_ps(0.0103592) ,x, _mm_set1_ps(0.00920833));
        let e = _mm_fmadd_ps(d,x, _mm_set1_ps(0.185));
        let f = _mm_fmadd_ps(e,x, _mm_set1_ps(0.05));

        _mm_fmadd_ps(f,x,_mm_set1_ps(1.0))
    }

    /**
       calculating feedback non-linear transconducance and compensated for r (-1) boosting
       non-linearity
      */
    #[inline] pub unsafe fn get_12db_self_osc(qfu: &mut QuadFilterUnitState) -> newton_raphson_12db::SelfOsc {

        let one = _mm_set1_ps(1.0);
        let k   = _mm_set1_ps(1.035);

        let push = qfu.coeff[Obxd12dBCoeff::SelfOscPush];
        let s1   = qfu.reg[ObxdParams::S1 as usize];

        let r_approx = diode_pair_resistance_approx(
            _mm_mul_ps( s1, _mm_set1_ps(0.0876))
        );

        newton_raphson_12db::SelfOsc {
            enabled_mask: _mm_cmpeq_ps(push, one),
            on_val:       _mm_sub_ps(r_approx, k),
            off_val:      _mm_sub_ps(r_approx, one),
        }
    }

    /// resolve 0-delay feedback
    #[inline] pub unsafe fn newton_raphson_12db(sample: __m128, qfu: &mut QuadFilterUnitState) -> __m128 {

        let self_osc = get_12db_self_osc(qfu);
        let t_cfb    = self_osc.cfb();
        let one      = _mm_set1_ps(1.0);
        let two      = _mm_set1_ps(2.0);
        let s1       = qfu.reg[ObxdParams::S1 as usize];
        let s2       = qfu.reg[ObxdParams::S2 as usize];
        let r        = qfu.coeff[Obxd12dBCoeff::R12];
        let g        = qfu.coeff[Obxd12dBCoeff::G12];

        // resolve linear feedback
        //
        // y = numerator / denominator
        // where n = (sample - 2*(s1*(r+t_cfb)) - g*s1  - s2)
        //       d = (1+ g*(2*(r+t_cfb)+ g))
        //
        //----------------------
        // ok i will make up some
        // variable names here... p1 and p2
        //
        // n  = (sample - 2*p1 - g*s1  - s2)
        //    where p1 = (s1*(r+t_cfb))
        //
        // d  = (1 + g*(p2 + g))
        //    where p2 = 2*(r+t_cfb)
        //
        //
        _mm_div_ps( 

            {//numerator
                let p1  = _mm_mul_ps(s1,    _mm_add_ps(r, t_cfb));
                let _0 = _mm_sub_ps(sample, _mm_mul_ps(two,p1));
                let _1 = _mm_sub_ps(_0,     _mm_mul_ps(g,s1));
                _mm_sub_ps(_1,s2)
            }, 

            {//denominator
                let p2 = _mm_mul_ps(two, _mm_add_ps(r, t_cfb));
                let _0 = _mm_mul_ps(g,   _mm_add_ps(p2, g));
                _mm_add_ps(one, _0)
            }
        )
    }

    #[inline] pub unsafe fn newton_raphson_r24_db(sample: __m128, lpc: __m128, qfu: &mut QuadFilterUnitState) -> __m128 
    {
        let s1 = qfu.reg[ObxdParams::S1 as usize];
        let s2 = qfu.reg[ObxdParams::S2 as usize];
        let s3 = qfu.reg[ObxdParams::S3 as usize];
        let s4 = qfu.reg[ObxdParams::S4 as usize];

        let g24 = qfu.coeff[Obxd24dBCoeff::G24];
        let r24 = qfu.coeff[Obxd24dBCoeff::R24];

        let ml: __m128 = one_over_one_plus_x(g24);

        let one = _mm_set1_ps(1.0);

        // float s = ( lpc * (lpc * (lpc * s1 + s2) + s3) +s4 ) * ml;
        let s = {
            let _0 = _mm_mul_ps(lpc, s1);
            let _1 = _mm_add_ps(_0,  s2);
            let _2 = _mm_mul_ps(lpc, _1);
            let _3 = _mm_add_ps(_2,  s3);
            let _4 = _mm_mul_ps(lpc, _3);
            let _5 = _mm_add_ps(_4,  s4);
            _mm_mul_ps(_5, ml)
        };

        // float g = lpc * lpc * lpc * lpc;
        let g: __m128 = x_to_the_fourth(lpc);

        // float y = (sample - r24 * s) / (1 + r24 * g);
        {
            let _0 = _mm_mul_ps(r24,     s);
            let _1 = _mm_sub_ps(sample, _0);
            let _2 = _mm_mul_ps(r24,     g);
            let _3 = _mm_add_ps(one,    _2);
            _mm_div_ps(_1,_3)
        }
    }

    #[inline] pub unsafe fn tptpc(state: &mut __m128, inp: __m128, cutoff: __m128) -> __m128 
    {
        let one = _mm_set1_ps(1.0);

        let v = {
            let _0 = _mm_sub_ps(inp, *state);
            let _1 = _mm_mul_ps(_0, cutoff);
            let _2 = _mm_add_ps(one, cutoff);
            _mm_div_ps(_1,_2)
        };

        let res: __m128 = _mm_add_ps(v, *state);

        *state = _mm_add_ps(res, v);

        res
    }
}
