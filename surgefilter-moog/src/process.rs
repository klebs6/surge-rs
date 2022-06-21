crate::ix!();

impl FilterProcessQuad for LpMoogFilter {

    #[cfg(target_arch = "x86_64")] 
    fn process_quad(&self, 
        qfu: &mut QuadFilterUnitState, 
        input: __m128) -> __m128 
    {

        //TODO: fix unknown coefficient names
        coeffidx![ C; X0, X1, X2, X3, X4, X5, X6, X7 ];

        unsafe {

            qfu.coeff[C::X0] = _mm_add_ps(qfu.coeff[C::X0], qfu.dcoeff[C::X0]);
            qfu.coeff[C::X1] = _mm_add_ps(qfu.coeff[C::X1], qfu.dcoeff[C::X1]);
            qfu.coeff[C::X2] = _mm_add_ps(qfu.coeff[C::X2], qfu.dcoeff[C::X2]);

            qfu.reg[C::X0] = softclip8_ps(
                _mm_add_ps(
                    qfu.reg[C::X0],
                    _mm_mul_ps(
                        qfu.coeff[C::X1], 
                        _mm_sub_ps(
                            _mm_sub_ps(
                                _mm_mul_ps(input, qfu.coeff[C::X0]),
                                _mm_mul_ps( 
                                    qfu.coeff[C::X2], 
                                    _mm_add_ps(qfu.reg[C::X3], qfu.reg[C::X4])
                                )
                            ),
                            qfu.reg[C::X0]
                        )
                    )
                )
            );

            qfu.reg[C::X1] = _mm_add_ps(
                qfu.reg[C::X1], 
                _mm_mul_ps(
                    qfu.coeff[C::X1], 
                    _mm_sub_ps(qfu.reg[C::X0], qfu.reg[C::X1])
                )
            );

            qfu.reg[C::X2] = _mm_add_ps(
                qfu.reg[C::X2], _mm_mul_ps(
                    qfu.coeff[C::X1], 
                    _mm_sub_ps(qfu.reg[C::X1], qfu.reg[C::X2])
                )
            );

            qfu.reg[C::X4] = qfu.reg[C::X3];
            qfu.reg[C::X3] = _mm_add_ps(
                qfu.reg[C::X3], 
                _mm_mul_ps(
                    qfu.coeff[C::X1], 
                    _mm_sub_ps(qfu.reg[C::X2], qfu.reg[C::X3])
                )
            );

            qfu.reg[(qfu.comb_write_position[0] & 3) as usize]
        }
    }
}

