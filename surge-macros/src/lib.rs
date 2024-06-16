///------------------------------------------------------------------------------------------
#[macro_export]
macro_rules! impl_trait_defaults {
    ($type:ty; $($trait:ident),* $(,)?) => {
        $(
            impl $trait for $type { }
        )*
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! no_update {

    ($ty:ident $(<$($life:lifetime),*>)?) => {

        impl$(<$( $life ),*>)? Update for $ty $(< $( $life ),* >)? { /* no-op */ }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! no_init {

    ($ty:ident $(< $( $life:lifetime ),* >)? ) => {

        impl $(<$($life),*>)? Initialize for $ty $(< $($life),*>)? { /* no-op */ }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! update_on_init {

    ($ty:ident $(< $( $life:lifetime ),* >)? ) => {

        impl $(<$( $life ),* >)? Initialize for $ty $(< $( $life ),* >)? {
            fn init(&mut self) {
                self.update();
            }
        }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! init_on_suspend {

    ($ty:ident $(<$($life:lifetime),*>)? ) => {

        impl $(<$($life),*>)? Suspend for $ty $(<$($life),*>)? { }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! default_default {

    ($ty:ident $(< $( $life:lifetime ),*>)? ) => {

        impl $(<$( $life ),*>)? Default for $ty $(< $( $life ),*>)? { 
            fn default() -> Self { Self { ..Default::default() } }
        }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! impl_default_new {

    ($ty:ident $(<$($life:lifetime),*>)? ) => {

        impl $(<$($life),*>)? $ty $(<$($life),*>)? {
            pub fn new() -> Self {
                Self {
                    ..Default::default()
                }
            }
        }
    }
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! no_op {

    ($ty:ident $(<$($life:lifetime),*>)? , $trait:ty ) => {

        impl $(<$($life),*>)? $trait for $ty $(<$($life),*>)? { }

    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! impl_process_ringout {
    ($ty:ident $(<$($life:lifetime),*>)?) => {
        impl surge_traits::GetRingout for $ty $(<$($life),*>)? {
            fn get_ringout(&self)         -> Ringout {
                self.ringout
            }
            fn get_ringout_counter(&self) -> NumberOfBlocks {
                match self.ringout {
                    Ringout::On { counter, decay } => counter,
                    _ => 0,
                }
            }
        }
        impl surge_traits::SetRingout for $ty $(<$($life),*>)? {
            fn ringout_counter_incr(&mut self) {
                match self.ringout {
                    Ringout::On { ref mut counter, ref mut decay } => *counter += 1,
                    _ => {},
                }
            }
            fn ringout_counter_reset(&mut self) {
                match self.ringout {
                    Ringout::On { ref mut counter, ref mut decay } => *counter = 0,
                    _ => {},
                }
            }
        }
        impl surge_traits::ProcessRingout for $ty $(<$($life),*>)? {}
    }
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! impl_return_level {
    ($ty:ident $(<$($life:lifetime),*>)?, $paramty:ty) => {
        impl surge_traits::GetReturnLevel for $ty $(<$($life),*>)? {
            fn returnlevel(&self) -> f32 {
                pvalf![self.params[<$paramty>::ReturnLevel]]
            }
        }
    }
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! effect {
    ($ty:ident $(<$($life:lifetime),*>)?, $paramty:ty ) => {
        surge_macros::impl_process_ringout![$ty $(<$($life),*>)?];
        surge_macros::impl_return_level![$ty $(<$($life),*>)?, $paramty];
        surge_macros::surge_base![$ty $(<$($life),*>)? , $paramty, Effect];
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! surge_base {

    ($ty:ident $(<$($life:lifetime),*>)?, $paramty:ty, $trait:ty ) => {

        impl $(<$($life),*>)? $trait for $ty $(<$($life),*>)? { }

        impl AssocParam for $ty $(<$($life),*>)? {
            type ParamType = $paramty;
        }

        impl $(<$($life),*>)? $ty $(<$($life),*>)? { 

            pub fn temposync(&self, p: $paramty) -> bool {
                self.params[p].get_temposync()
            }

            pub fn pvalf(&self, p: $paramty) -> f32 {
                let result: f32 = pvalf![self.params[p]];
                result
            }

            pub fn pvali(&self, p: $paramty) -> i32 {
                let result: i32 = pvali![self.params[p]];
                result
            }

            pub fn pvalf_extended(&self, p: $paramty) -> f32 {
                let f = self.pvalf(p);
                p.get_extended(f)
            }

        }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! has_timeunit {

    ($ty:ident $(<$($life:lifetime),*>)?, $paramty:ty ) => {

        impl$(<$($life),*>)? $ty $(< $( $life ),*>)? { 
            pub fn maybe_temposyncratio(&self, p: $paramty) -> f32 {
                match self.params[p].get_temposync() {
                    true => self.time_unit.temposyncratio(),
                    false => 1.0,
                }
            }

            pub fn maybe_temposyncratio_inv(&self, p: $paramty) -> f32 {
                match self.params[p].get_temposync() {
                    true =>  self.time_unit.temposyncratio_inv(),
                    false => 1.0,
                }
            }
        }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! fixed_ringout {

    ($ty:ident $(<$($life:lifetime),*>)?, $x:expr  ) => {

        impl $(<$($life),*>)? ProcessRingout for $ty $(<$($life),*>)? {

            const FIXED_RINGOUT: Option<NumberOfBlocks> = Some($x);
        }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! variable_ringout {

    ($ty:ident $(<$($life:lifetime),*>)?) => {

        impl $(<$($life),*>)? ProcessRingout for $ty $(<$($life),*>)? {

            const FIXED_RINGOUT: Option<NumberOfBlocks> = None;
            fn get_ringout(&self) -> Option<NumberOfBlocks> { 
                self.ringout_time 
            } 
            fn set_ringout(&mut self, x: NumberOfBlocks) { 
                self.ringout_time = Some(x); 
            }
        }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! spawn_handle {
    ($base:ident,$handle:ident,$lt:lifetime) => {

        #[derive(Debug,Clone)] 
        pub struct $handle<$lt> {
            inner: Rc<RefCell<$base<$lt>>>,
        }
    }
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! wt_flag {
    ($self:ident, $f:ty) => {{
        let flags = $self.wave_wavetable.header.flags.clone();
        !(flags & WaveTableFlags::IS_SAMPLE).is_empty() 
    }}
}

///------------------------------------------------------------------------------------------
#[macro_export]
macro_rules! allow_display {

    ($ty:ident $(<$($life:lifetime),*>)?, $b:expr) => {

        impl AllowDisplay for $ty $(<$($life),*>)? {

            fn allow_display(&self) -> bool {
                $b
            }
        }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export]
macro_rules! oscillator {

    ($ty:ident $(<$($life:lifetime),*>)?, $paramty:ty) => {
        surge_macros::surge_base![$ty $(<$($life),*>)? , $paramty, Oscillator];
        surge_macros::impl_oscillator_stereo_out![$ty $(<$($life),*>)?];
        surge_macros::impl_oscillator_param_access![$ty $(<$($life),*>)?];
    };
}

///------------------------------------------------------------------------------------------
#[macro_export]
macro_rules! impl_oscillator_stereo_out {

    ($ty:ident $(<$($life:lifetime),*>)? ) => {

        impl $(<$($life),*>)? OscillatorStereoOut for $ty $(<$($life),*>)? {

            fn out_l(&mut self) -> *mut f32 {
                self.out.l.as_mut_ptr()
            }
            fn out_r(&mut self) -> *mut f32 {
                self.out.r.as_mut_ptr()
            }
        }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export]
macro_rules! impl_oscillator_param_access {

    ($ty:ident $(<$($life:lifetime),*>)? ) => {

        impl $(<$($life),*>)? surge_traits::OscillatorParamAccess for $ty $(<$($life),*>)? {

            //TODO: switch const/mut semantics
            fn osc_params_const(&self, name: surge_types::OscillatorParam) -> &surge_types::OscillatorParamRT {
                &self.osc_params[name]
            }
            fn osc_params(&mut self, name: surge_types::OscillatorParam) -> &mut surge_types::OscillatorParamRT {
                &mut self.osc_params[name]
            }
        }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! master_osc {
    ($self:ident,$s:expr) => ({
        let master_osc: f32 = unsafe { 
            *$self.master_osc.add($s) 
        };
        master_osc
    })
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! compare_by {

    ($ty:ident $(< $life:lifetime >)?, $f:ident) => {

        impl $(< $life >)?  PartialOrd for 
            $ty $(< $life >)? 
            {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }

        impl $(< $life >)?  Ord for 
            $ty $(< $life >)? 
            {
                fn cmp(&self, other: &Self) -> Ordering {
                    self.$f.cmp(&other.$f)
                }
            }

        impl $(< $life >)?  PartialEq for 
            $ty $(< $life >)? 
            {
                fn eq(&self, other: &Self) -> bool {
                    self.$f == other.$f
                }
            }

        impl $(< $life >)?  Eq for 
            $ty $(< $life >)?  { }
    };
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! encapsulate_integral {
    ($outer:tt, $inner:tt) => {
        #[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
        pub struct $outer(pub $inner);
        impl std::ops::Add for $outer {
            type Output = $outer;
            fn add(self, x: $outer) -> Self::Output {
                $outer(self.0 + x.0)
            }
        }
        impl std::ops::Sub for $outer {
            type Output = $outer;
            fn sub(self, x: $outer) -> Self::Output {
                $outer(self.0 - x.0)
            }
        }
        impl std::ops::Mul for $outer {
            type Output = $outer;
            fn mul(self, x: $outer) -> Self::Output {
                $outer(self.0 * x.0)
            }
        }
        impl std::ops::Div for $outer {
            type Output = $outer;
            fn div(self, x: $outer) -> Self::Output {
                $outer(self.0 / x.0)
            }
        }
    }
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! encapsulate_string {
    ($outer:tt) => {
        #[derive(Debug,Clone,Eq,PartialEq)]
        pub struct $outer(pub String);
    }
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! encapsulate_bool {
    ($outer:tt) => {
        #[derive(Debug,PartialEq,Eq,Copy,Clone)]
        pub struct $outer(pub bool);
        impl std::ops::Not for $outer {
            type Output = bool;
            fn not(self) -> Self::Output {
                !self.0
            }
        }
    }
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! block_size_inv {
    ($n:expr) => {{
        1.0 / ($n as f32)
    }}
}
#[macro_export] macro_rules! block_size_quad {
    ($n:expr) => {{
        $n >> 2
    }}
}
#[macro_export] macro_rules! block_size_oversample {
    ($n:expr) => {{
        let oversampling_factor = 2;
        $n * oversampling_factor
    }}
}
#[macro_export] macro_rules! block_size_oversample_inv {
    ($n:expr) => {{
        1.0 / (block_size_oversample![$n] as f32)
    }}
}

#[macro_export] macro_rules! block_size_oversample_quad {
    ($n:expr) => {{
        block_size_oversample![$n] >> 2
    }}
}

///------------------------------------------------------------------------------------------
#[macro_export] macro_rules! encapsulate_float {
    ($outer:tt, $inner:tt) => {
        #[derive(Debug,Copy,Clone,PartialEq,PartialOrd)]
        pub struct $outer(pub $inner);
        impl std::ops::Add for $outer {
            type Output = $outer;
            fn add(self, x: $outer) -> Self::Output {
                $outer(self.0 + x.0)
            }
        }
        impl std::ops::Sub for $outer {
            type Output = $outer;
            fn sub(self, x: $outer) -> Self::Output {
                $outer(self.0 - x.0)
            }
        }
        impl std::ops::Mul for $outer {
            type Output = $outer;
            fn mul(self, x: $outer) -> Self::Output {
                $outer(self.0 * x.0)
            }
        }
        impl std::ops::Div for $outer {
            type Output = $outer;
            fn div(self, x: $outer) -> Self::Output {
                $outer(self.0 / x.0)
            }
        }
    }
}
