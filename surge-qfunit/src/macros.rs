ix!();

macro_rules! qfc_assert_reasonable_audio_float {
    ($x:expr) => ({
        if cfg!(debug_assertions) {
            assert!( $x < 32.0 && $x > -32.0 );
        } 
    });
}

macro_rules! v_madd {
    ($a:expr, $b:expr, $c:expr) => {
        _mm_add_ps(_mm_mul_ps($a,$b),$c)
    }
}
