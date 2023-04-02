crate::ix!();

enhanced_enum!{

    // This is an enhanced enumeration that
    // defines different types of non-linear state
    // filters, including low-pass, high-pass,
    // band-pass, notch, and all-pass. 
    //
    // It allows you to define these filter types
    // as values of the enumeration, making them
    // easier to use in your code.
    //
    NLSFType {
        LowPass,
        HighPass,
        BandPass,
        Notch,
        Allpass,
    }
}
