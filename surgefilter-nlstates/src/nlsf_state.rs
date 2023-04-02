crate::ix!();

// This is another coefficient index that defines
// the different state variables used in
// a non-linear state filter. 
//
// It defines `Z1` and `Z2` as the first and
// second `z-1` state for the first stage, `Z3`
// and `Z4` as the first and second `z-1` state
// for the second stage, and so on up to the
// fourth stage (`Z7` and `Z8`).
//
coeffidx!{
    NLSFState;

    // 1st z-1 state for first  stage
    Z1, 

    // 2nd z-1 state for first  stage
    Z2, 

    // 1st z-1 state for second stage
    Z3, 

    // 2nd z-1 state for second stage
    Z4, 

    // 1st z-1 state for third  stage
    Z5, 

    // 2nd z-1 state for third  stage
    Z6, 

    // 1st z-1 state for fourth stage
    Z7, 

    // 2nd z-1 state for fourth stage
    Z8 
}
