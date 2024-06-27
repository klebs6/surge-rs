crate::ix!();

#[derive(Debug)]
pub struct FBP {
      pub gain:            f32,
      pub fb:              f32,
      pub mix1:            f32,
      pub mix2:            f32,
      pub out_l:           f32,
      pub out_r:           f32,
      pub out_2l:          f32,
      pub out_2r:          f32,
      pub drive:           f32,
      pub ws_lpf:          f32,
      pub feedback_line_l: f32,
      pub feedback_line_r: f32,
      pub delay:           [[f32; MAX_FB_COMB + FIR_IPOL_N]; 4],
      pub fu:              [FU; 4],
}

impl Default for FBP {
    fn default() -> Self {
        Self {
            gain:            0.0,
            fb:              0.0,
            mix1:            0.0,
            mix2:            0.0,
            out_l:           0.0,
            out_r:           0.0,
            out_2l:          0.0,
            out_2r:          0.0,
            drive:           0.0,
            ws_lpf:          0.0,
            feedback_line_l: 0.0,
            feedback_line_r: 0.0,
            delay:           [[0.0; MAX_FB_COMB + FIR_IPOL_N]; 4],
            fu:              [
                FU::default(), 
                FU::default(), 
                FU::default(), 
                FU::default(), 
            ],
        }
    }
}

#[derive(Debug)]
pub struct FU {
    pub coeff:               [f32; N_COEFFMAKER_COEFFS],
    pub reg:                 [f32; N_FILTER_REGISTERS],
    pub comb_write_position: u32,
    pub ty:                  FilterType,

    ///used for comparison with the last run
    pub subty: FilterSubType, 
}

impl Default for FU {
    fn default() -> Self {
        Self {
            coeff:                [0.0; N_COEFFMAKER_COEFFS],
            reg:                  [0.0; N_FILTER_REGISTERS],
            comb_write_position:  0,
            ty:                   FilterType::Off,
            subty:                FilterSubType::SVF, 
        }
    }
}

impl Initialize for FU {

    fn init(&mut self) -> Result<(),SurgeError> {

        self.coeff               = [0.0; N_COEFFMAKER_COEFFS];
        self.reg                 = [0.0; N_FILTER_REGISTERS];
        self.comb_write_position = 0;
        self.ty                  = FilterType::Off;
        self.subty               = FilterSubType::SVF;

        Ok(())
    }
}
