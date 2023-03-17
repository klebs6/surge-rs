crate::ix!();

/// This is a Rust macro definition that defines
/// a macro named `lipol_ps_sse_block`. Macros are
/// a way to write code that generates other code. 
///
/// This macro generates code that performs
/// a specific calculation using SIMD
/// instructions. The macro takes four arguments,
/// including an identifier `$self`, an expression
/// `$nquads`, an expression `$n`, and a code
/// block `$code`. 
///
/// The code block is the most important part of
/// this macro, as it contains the actual code
/// that will be generated.
///
#[macro_export] macro_rules! lipol_ps_sse_block {
    // nquads must be multiple of 4
    ($self:ident, $nquads:expr, $n:expr; $code:expr) => (

        // These two lines declare and initialize
        // two mutable variables named `y1` and
        // `dy`. 
        //
        // These variables are 128-bit SIMD
        // vectors of floating-point numbers (a
        // data type defined in the
        // `std::arch::x86_64` module). 
        //
        // The `z128![]` macro initializes the
        // variables to zero.
        //
        let mut y1: __m128 = z128![];
        let mut dy: __m128 = z128![];

        // This line calls a method named
        // `initblock` on the `$self` object
        // (which is passed in as an argument to
        // the macro). 
        //
        // This method initializes the `y1` and
        // `dy` variables with appropriate values
        // for the specific calculation being
        // performed.
        //
        $self.initblock(&mut y1, &mut dy);

        // These two lines perform SIMD
        // calculations on the `y1` and `dy`
        // variables. 
        //
        // The `_mm_add_ps` function adds the
        // values of `y1` and `dy` element-wise,
        // and the result is stored in a new
        // variable named `y2`. 
        //
        // The `_mm_mul_ps` function multiplies
        // the values of `dy` by a constant value
        // (defined using the `m128_two![]`
        // macro), and the result is stored back
        // in the `dy` variable.
        //
        let y2 = _mm_add_ps(y1, dy);
        dy = _mm_mul_ps(dy, m128_two![]);

        // This code block contains a Rust `match`
        // expression that selects between two
        // different loops based on the value of
        // `$n`. 
        //
        // If `$n` is `2`, then the loop iterates
        // over the range `0..$nquads`
        //
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
