crate::ix!();

#[derive(Derivative)] #[derivative(Debug)]
pub struct IIRFilter {

    #[derivative(Debug="ignore")]
    pub tuner:   TunerHandle,

    #[derivative(Debug="ignore")]
    pub tables:  TablesHandle,

    pub subtype: FilterSubType,
    pub iirtype: FilterTypeIIR,
    pub pole:    PoleType,
}

impl IIRFilter {
    pub fn new_default(tables: &TablesHandle, tuner: &TunerHandle) -> Self 
    {
        Self {
            tuner:   tuner.clone(),
            tables:  tables.clone(),
            subtype: FilterSubType::Rough,
            iirtype: FilterTypeIIR::LowPass,
            pole:    PoleType::FourPole,
        }
    }
}

impl CoeffMake for IIRFilter {
    fn coeff_make(&self, mut freq: f32, reso: f32) -> [f32; N_COEFFMAKER_COEFFS]
    {
        let gain: f64 = crate::resoscale(reso.into(), self.subtype);

        boundfreq(&mut freq);

        let q2inv = FilterCoeffs::q2inv(
            reso.into(), freq.into(), self.subtype, self.pole
        );

        let (cosi, sinu): 
            (f64, f64) = self.tuner.note_to_omega::<f64,true>(freq.into());

        let alpha = FilterCoeffs::alpha(sinu, cosi, q2inv, self.subtype);

        let coeffs = match self.iirtype {
            FilterTypeIIR::LowPass  => FilterCoeffs::lowpass(alpha,cosi),
            FilterTypeIIR::BandPass => FilterCoeffs::bandpass(alpha, cosi, q2inv),
            FilterTypeIIR::HighPass => FilterCoeffs::highpass(alpha, cosi),
        };

        let clipscale = self.tables.clipscale(freq as f32, self.subtype);

        match self.subtype {
            FilterSubType::Smooth => coeffs.to_normalized_lattice(gain, clipscale as f64),
            _                     => coeffs.to_coupled_form(gain, clipscale as f64),
        }
    }
}
