crate::ix!();

/**
  |# Safety
  |
  |need to be able to access distance k away from
  |out_r and out_l
  */
pub unsafe fn qfc_write_outputs(
    qfcs:  &mut QuadFilterChainState,
    x:     __m128,
    k:     usize,
    out_r: *mut f32,
    out_l: *mut f32)
{
    (*qfcs).out_l = _mm_add_ps((*qfcs).out_l, (*qfcs).dout_l);
    (*qfcs).out_r = _mm_add_ps((*qfcs).out_r, (*qfcs).dout_r);

    let new_out_l: __m128 = _mm_mul_ps(x, (*qfcs).out_l);
    let new_out_r: __m128 = _mm_mul_ps(x, (*qfcs).out_r);

    _mm_store_ss(out_l.add(k), _mm_add_ss(_mm_load_ss(out_l.add(k)), sum_ps_to_ss(new_out_l)));
    _mm_store_ss(out_r.add(k), _mm_add_ss(_mm_load_ss(out_r.add(k)), sum_ps_to_ss(new_out_r)));
}

/**
  |# Safety
  |
  |need to be able to access distance k away from
  |out_r and out_l
  */
pub unsafe fn qfc_write_outputs_dual(
    qfcs:  &mut QuadFilterChainState,
    x:     __m128,
    y:     __m128,
    k:     usize,
    out_r: *mut f32,
    out_l: *mut f32)
{
    (*qfcs).out_l  = _mm_add_ps((*qfcs).out_l, (*qfcs).dout_l);
    (*qfcs).out_l  = _mm_add_ps((*qfcs).out_l, (*qfcs).dout_l);
    (*qfcs).out_r  = _mm_add_ps((*qfcs).out_r, (*qfcs).dout_r);
    (*qfcs).out_2l = _mm_add_ps((*qfcs).out_2l, (*qfcs).dout_2l);
    (*qfcs).out_2r = _mm_add_ps((*qfcs).out_2r, (*qfcs).dout_2r);

    let new_out_l: __m128 = v_madd!(x, (*qfcs).out_l, _mm_mul_ps(y, (*qfcs).out_2l));
    let new_out_r: __m128 = v_madd!(x, (*qfcs).out_r, _mm_mul_ps(y, (*qfcs).out_2r));

    _mm_store_ss(out_l.add(k), _mm_add_ss(_mm_load_ss(out_l.add(k)), sum_ps_to_ss(new_out_l)));
    _mm_store_ss(out_r.add(k), _mm_add_ss(_mm_load_ss(out_r.add(k)), sum_ps_to_ss(new_out_r)));
}
