ix!();

/*
  |this is implemented as a bunch of function
  |pointers which could potentially be passed back
  |up to the caller
  |
  |TODO: eventually make a type of param which is
  |just a selector for one of several enum options
  */
enhanced_enum!{
    WaveshaperParam {
        Type,
        Drive,
    }
}

impl Param for WaveshaperParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Filter } 
    /* TODO */
}

#[derive(Debug)]
pub struct WaveshaperUnit {
    pub params: WaveshaperParamArray::<ParamRT::<WaveshaperParam>>,
}

impl Default for WaveshaperUnit {
    fn default() -> Self {
        todo!();

    }
}

#[derive(Debug)]
pub struct WaveshaperState {
    pub tables: TablesHandle,
}

