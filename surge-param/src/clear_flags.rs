crate::ix!();

pub trait ClearFlags {
    fn clear_flags(&mut self) ;
}

impl<P: ParameterInterface> ClearFlags for ParamRT<P> {

    fn clear_flags(&mut self) {

        self.set_extend_range(false);
        self.set_absolute(false);
        self.set_temposync(false);
        self.set_snap(false);
    }
}
