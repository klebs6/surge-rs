crate::ix!();

#[inline] pub fn bend(x: f32, b: f32) -> f32 { 
    (1.0 + b) * x - b * x * x * x 
}

/// panning that always lets both channels through unattenuated (seperate hard-panning)
#[inline] pub fn trixpan(l: &mut f32, r: &mut f32, x: f32) {

    if x < 0.0 {
        *l -= x * *r;
        *r *= 1.0 + x;
    } 
    else {
        *r += x * *l;
        *l *= 1.0 - x;
    }
}

enhanced_enum![ 
    StereoChannel {
        Left,
        Right,
    }
];
