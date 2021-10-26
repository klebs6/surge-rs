ix!();

use crate::*;

pub fn get_fn_process_quad(fbc: FilterBlockConfiguration, x: &FbqGlobal) 
-> Box<FBQFPtr> 
{
    let a  = x.fu1ptr.is_some();
    let b  = x.fu2ptr.is_some();
    let ws = x.wsptr.is_some();

    macro_rules! dispatch{
        ($x:ident, $a:ident, $b:ident, $c:ident) => {
            match ($a, $b, $c)  {
                (false, false, false) => $x::<false,false,false>, 
                (false, false, true ) => $x::<false,false,true>, 
                (false, true,  false) => $x::<false,true,false>, 
                (false, true,  true ) => $x::<false,true,true>, 
                (true,  false, false) => $x::<true,false,false>, 
                (true,  false, true ) => $x::<true,false,true>, 
                (true,  true,  false) => $x::<true,true,false>, 
                (true,  true,  true ) => $x::<true,true,true>, 
            }
        };
    }
    use crate::{
        do_serial1,
        do_serial2,
        do_serial3,
        do_dual1,
        do_dual2,
        do_stereo,
        do_ring,
        do_wide
    };
    box match fbc {
        FilterBlockConfiguration::Serial1 => { dispatch![do_serial1, a, ws, b] },
        FilterBlockConfiguration::Serial2 => { dispatch![do_serial2, a, ws, b] },
        FilterBlockConfiguration::Serial3 => { dispatch![do_serial3, a, ws, b] },
        FilterBlockConfiguration::Dual1   => { dispatch![do_dual1,   a, ws, b] },
        FilterBlockConfiguration::Dual2   => { dispatch![do_dual2,   a, ws, b] },
        FilterBlockConfiguration::Stereo  => { dispatch![do_stereo,  a, ws, b] },
        FilterBlockConfiguration::Ring    => { dispatch![do_ring,    a, ws, b] },
        FilterBlockConfiguration::Wide    => { dispatch![do_wide,    a, ws, b] },
    }
}

#[test] fn does_this_work() {
    //you betcha
    //^old. now it is broken again :)

    /*
    let a: bool = true;
    let b: bool = false;
    let c: bool = true;
    let mut x = FilterBlockConfiguration::Wide;
    x.process_fb_quad(a,b,c)(
        std::ptr::null_mut(),
        std::ptr::null_mut() , 
        std::ptr::null_mut(), 
        std::ptr::null_mut());
    */

}

