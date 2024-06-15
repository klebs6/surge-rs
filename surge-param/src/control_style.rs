crate::ix!();

pub trait GetControlStyle {
    fn control_style(&self)
        -> ControlStyle { ControlStyle::OFF }
}

impl<P: ParameterInterface + ?Sized> GetControlStyle for ParamRT<P> {

    delegate!{
        to self.delegate {
            fn control_style(&self) -> ControlStyle;
        }
    }
}

//TODO: this is a little fishy
bitflags! {
    pub struct ControlStyle: u32 {
        const OFF        = 0b00000000000000000000000000000000;
        const HORIZONTAL = 0b00000000000000000000000000000001;
        const VERTICAL   = 0b00000000000000000000000000000010;
        const BIPOLAR    = 0b00000000000000001000000000000000;
        const WHITE      = 0b00000000000000010000000000000000;
        const SEMITONE   = 0b00000000000000100000000000000000;
        const MINI       = 0b00000000000001000000000000000000;
        const META       = 0b00000000000010000000000000000000;
        const EASY       = 0b00000000000100000000000000000000;
        const HIDE       = 0b00000000001000000000000000000000;
        const NOPOPUP    = 0b00000000010000000000000000000000;
    }
}
