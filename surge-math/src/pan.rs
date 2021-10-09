ix!();

// valid range -2 .. 2 (> +- 1 is inverted phase) 
#[inline] pub fn megapan_left(mut pos: f32 ) -> f32 {

    if pos > 2.0 {
        pos = 2.0;
    }

    if pos < -2.0 {
        pos = -2.0;
    }

    1.0 - 0.75 * pos - 0.25 * pos * pos
}

#[inline] pub fn megapan_right(mut pos: f32 ) -> f32 {

    if pos > 2.0 {
        pos = 2.0;
    }

    if pos < -2.0 {
        pos = -2.0;
    }

    1.0 + 0.75 * pos - 0.25 * pos * pos
}


