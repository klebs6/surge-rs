crate::ix!();

#[derive(Debug,Clone)]
#[parameter_interface]
pub struct ParamRT<P: ParameterInterface + ?Sized> {
    val:                    PData,//want interior mutability
    modulation_delta:       PData,//want interior mutability
    midictrl:               Option<i32>,

    //are these compile time values?
    per_voice_processing:   bool,
    temposync:              bool,
    extend_range:           bool,
    absolute:               bool,
    snap:                   bool,
    pub(crate) delegate:    Box<P>,
}

impl<P: ParameterInterface + ?Sized> SetSnap for ParamRT<P> {

    fn set_snap(&mut self, snap: bool) {
        self.snap = snap;
    }
}

impl<P: ParameterInterface + ?Sized> ParamRT<P> {

    pub fn get_midictrl(&self) -> Option<i32> {
        self.midictrl
    }

    pub fn get_per_voice_processing(&self) -> bool {
        self.per_voice_processing
    }

    pub fn get_value(&self) -> PData {
        self.val
    }

    pub fn set_value(&mut self, value: PData) {
        self.val = value;
    }

    pub fn get_temposync(&self) -> bool {
        self.temposync
    }

    pub fn set_temposync(&mut self, x: bool) {
        self.temposync = x;
    }

    pub fn set_extend_range(&mut self, x: bool) {
        self.extend_range = x;
    }

    pub fn set_absolute(&mut self, x: bool) {
        self.absolute = x;
    }
}

impl<P: ParameterInterface + ?Sized> GetModulationVal for ParamRT<P> {

    fn get_modulation_val(&self) -> PData {
        self.modulation_delta
    }
}

impl<P: ParameterInterface + ?Sized> SetModulationVal for ParamRT<P> {

    fn set_modulation_val(&mut self, val: PData) {
        self.modulation_delta = val;
    }
}

impl<P: ParameterInterface> ParamRT<P> {

    pub fn default_modulation_delta(default_val: PData) -> PData {
        match default_val {
            PData::Float(_x) => PData::Float(0.0),
            PData::Int(_x)   => PData::Int(0),
            PData::Bool(_x)  => PData::Bool(false),
        }
    }

    pub fn new( delegate: P ) -> Self {

        let default_val = delegate.default_value();

        let mut x = Self {
            delegate:             Box::new(delegate),
            val:                  default_val,
            modulation_delta:     Self::default_modulation_delta(default_val),
            midictrl:             None,
            per_voice_processing: true,
            temposync:            false,
            extend_range:         false,
            absolute:             false,
            snap:                 false,
        };
        let force_integer = false;
        x.bound_value(force_integer);
        x
    }
}
