crate::ix!();

#[inline] pub fn amp_to_linear(mut x: f32) -> f32 {
    x = x.max(0.0);
    x * x * x
}

#[inline] pub fn linear_to_amp(x: f32) -> f32 {
    limit_range(x, 0.0000000001, 1.0).powf(1.0 / 3.0) 
}

#[inline] pub fn amp_to_db(x: f32) -> f32 {
    let val = 6.0 * 3.0 * x.log2();
    limit_range( val, -192.0, 96.0 )
}

#[inline] pub fn db_to_amp(x: f32) -> f32 {
    let base: f32 = 10.0 / 3.0;
    let exp: f32 = 0.05 * x;
    limit_range( base.powf(exp), 0.0, 1.0 )
}

#[inline] pub fn db_to_scamp(input: f32)  -> f32 // ff rev2
{
   let v: f32 = 10.0_f32.powf( -0.05 * input);
   limit_range(v, 0.0, 1.0)
}

#[inline] pub fn linear_to_db(input: f64) -> f64 
{
    20.0 * input.log10()
}

#[inline] pub fn db_to_linear(input: f64) -> f64 
{
    10.0_f64.powf(0.05 * input)
}

#[inline] pub fn timecent_to_seconds(input: f32) -> f32 
{
    2.0_f32.powf(input / 1200.0)
}

#[inline] pub fn seconds_to_envtime(input: f32)  -> f32 // ff rev2
{
    let base: f32 = input / 30.0;
    let v: f32 = base.powf( 1.0 / 3.0 );
    limit_range(v, 0.0, 1.0)
}

#[inline] pub fn timecent_to_envtime(input: f32) -> f32 
//TODO: is this actually doing the right thing?
// return seconds_to_envtime(timecent_to_seconds(in));
{
    input / 1200.0
}

#[cfg(test)]
mod convert_tests {

    use super::*;

    #[test] fn amp_linear() {
        let lin = 0.5;
        let amp = linear_to_amp(lin);
        let back = amp_to_linear(amp);
        println!("lin: {}, amp: {}", lin, amp);
        assert_eq!(lin,back);

        let amp = 0.7937005;
        let lin = amp_to_linear(amp);
        let back = linear_to_amp(lin);
        println!("amp: {}, lin: {}", amp, lin);
        assert_eq!(amp,back);
    }

    #[test] fn amp_db() {

        for amp in 0..100 {
            let amp = amp as f32 / 100.0;
            let db = amp_to_db(amp);
            println!("amp: {}, db: {}",amp,db);
        }

        //according to this, any dB > 0 is saturated to 1
        //is this actually what we want?
        for db in -192..96 {
            let amp: f32 = db_to_amp(db as f32);
            println!("db: {}, amp: {}",db,amp);
        }
    }
}

pub fn int7_to_bipolar_float(x: i32) -> f32
{
    match x {
        _ if x > 64 => ((x - 64) as f32) * (1.0 / 63.0),
        _ if x < 64 => ((x - 64) as f32) * (1.0 / 64.0),
        _ => 0.0,
    }
}
