ix!();

use crate::{
    DualDelay,
    DUAL_DELAY_MAX_DELAY_LENGTH,
};

impl DualDelay {

    pub fn process_time(&mut self) {
        self.time_l.process();
        self.time_r.process();
    }

    pub fn do_dualdelay_block(&mut self, k: usize) {

        macro_rules! get_dtime {
            ($chan:ident) => {
                maxi(
                    BLOCK_SIZE as i32, 
                    mini(
                        self.$chan.v as i32, 
                        (DUAL_DELAY_MAX_DELAY_LENGTH - FIR_IPOL_N - 1) as i32
                    )
                )
            }
        }

        macro_rules! get_rp {
            ($dtime:ident) => {
                ((self.wpos - $dtime + k as i32) - FIR_IPOL_N as i32) 
                    & (DUAL_DELAY_MAX_DELAY_LENGTH as i32 - 1)
            }
        }

        macro_rules! get_sinc {
            ($chan:ident, $dtime:ident) => {
                FIR_IPOL_N as i32 *
                    limit_range(
                        (FIR_IPOL_M as f32 * (($dtime + 1) as f32 - self.$chan.v)) as i32, 
                        0, 
                        FIR_IPOL_M as i32 - 1
                    )
            }
        }

        macro_rules! sinc_product {
            ($axidx:expr,$sinc:expr,$rp:expr) => {
                _mm_mul_ps(
                    _mm_load_ps(
                        self.tables.sinctable_1x_ptr($sinc)
                    ), 
                    _mm_loadu_ps(
                        //TODO: is this index_axis_mut call correct?
                        self.buffer.index_axis_mut(Axis(0),$axidx).as_mut_ptr().offset(($rp) as isize)
                    )
                )
            }
        }

        self.process_time();

        let i_dtime_l: i32 = get_dtime![time_l];
        let i_dtime_r: i32 = get_dtime![time_r];

        let rp_l:      i32 = get_rp![i_dtime_l];
        let rp_r:      i32 = get_rp![i_dtime_r];

        let sinc_l:    i32 = get_sinc![time_l, i_dtime_l];
        let sinc_r:    i32 = get_sinc![time_r, i_dtime_r];

        unsafe {

            //------------------------------------------------
            let mut l: __m128 = sinc_product![0,sinc_l,rp_l];

            l = _mm_add_ps(l, 
                sinc_product![0,sinc_l + 4, rp_l + 4]
            );

            l = _mm_add_ps(l, 
                sinc_product![0, sinc_l + 8, rp_l + 8] 
            );

            l = sum_ps_to_ss(l);

            //------------------------------------------------
            let mut r: __m128 = sinc_product![1, sinc_r, rp_r];

            r = _mm_add_ps( 
                r, 
                sinc_product![1, sinc_r + 4, rp_r + 4] 
            );

            r = _mm_add_ps( 
                r, 
                sinc_product![1, sinc_r + 8, rp_r + 8] 
            );

            r = sum_ps_to_ss(r);

            _mm_store_ss(self.scratch_left.as_mut_ptr().add(k), l);
            _mm_store_ss(self.scratch_right.as_mut_ptr().add(k), r);
        }
    }
}
