#![feature(stdarch)]

macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( use crate::{ imports::* , };) }

#[macro_use] pub mod imports;

/**
  |this macro is used to clean up the common
  |initialization and looping code found in many of
  |the functions in this crate.  there are two loop
  |variants, by two and by eight, as exposed by the
  |"match"
  */
macro_rules! lipol_ps_sse_block {
    // nquads must be multiple of 4
    ($self:ident, $nquads:expr, $n:expr; $code:expr) => (

        let mut y1: __m128 = z128![];
        let mut dy: __m128 = z128![];

        $self.initblock(&mut y1, &mut dy);

        let y2 = _mm_add_ps(y1, dy);
        dy = _mm_mul_ps(dy, m128_two![]);

        match $n {
            2 => {
                for i in (0..$nquads).step_by(2) { 
                    $code(i as usize, y1, y2, dy);
                }
            },
            8 => {
                let n: usize = $nquads << 2;
                for i in (0..n).step_by(8) { 
                    $code(i as usize, y1, y2, dy);
                }
            },
            _ => unreachable!()
        }
        );
}

x![add];
x![fade];
x![lipol];
x![lipol_ps];
x![mac];
x![multiply];
x![trixpan];
