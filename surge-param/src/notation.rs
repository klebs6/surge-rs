crate::ix!();

#[inline] pub fn get_notename( i_value: i32 ) -> String
{
    let octave: i32 = (i_value / 12) - 2;
    let notenames: [&'static str;  12] =  [
        "C ", "C#", "D ", "D#", "E ", "F ", "F#", "G ", "G#", "A ", "A#", "B "
    ];
    let idx: usize = (i_value % 12).try_into().unwrap();
    format!("{}{}", notenames[idx], octave)
}

//TODO find a better way
pub fn tempo_sync_notation_value(val: f32) -> String {

    let (mut integral, mut fractional) = split_float(val);

    if fractional < 0.0 {
        fractional += 1.0;
        integral -= 1.0;
    }

    let mut d: f32;
    let mut q: f32;
    let mut nn: String;
    let t:  String;

    match val >= 1.0 {
        true => {

            q  = 2.0_f32.powf( val - 1.0 );
            nn = "whole".into();

            match q {
                _ if q >= 3.0 => {
                    return format!("{:0.2} whole notes", q);
                },

                _ if q >= 2.0 => {
                    nn = "double whole".into();
                    q /= 2.0;
                },
                _ => {},
            }

            t = match q {
                _ if q < 1.3 => {
                    "note".into()
                },
                _ if q < 1.4 => {
                    if nn.eq("whole") { 
                        nn = "1/2".into(); 
                    }
                    "triplet".into()
                },
                _ => { "dotted".into() },
            };

        },
        false => {

            d = 2.0_f32.powf( - (integral - 2.0) );
            q = 2.0_f32.powf( fractional + 1.0 );

            t = match q {
                _ if q < 1.3 => { "note".into() },
                _ if q < 1.4 => { d /= 2.0; "triplet".into() },
                _            => { "dotted".into() },
            };

            nn = match (d - 1.0).abs() < f32::EPSILON {
                true  => "whole".into(),
                false => format!("1/{:0.5}", d),
            };
        }
    }
    return format!("{} {}", nn, t)
}
