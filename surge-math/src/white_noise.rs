ix!();

use crate::*;

/**
  | In signal processing, white noise is
  | a random signal having equal intensity
  | at different frequencies, giving it
  | a constant power spectral density.
  | White noise refers to a statistical
  | model for signals and signal sources,
  | rather than to any specific signal.
  | White noise draws its name from white
  | light, although light that appears
  | white generally does not have a flat
  | power spectral density over the visible
  | band.
  | 
  | In discrete time, white noise is a discrete
  | signal whose samples are regarded as
  | a sequence of serially uncorrelated
  | random variables with zero mean and
  | finite variance; a single realization
  | of white noise is a random shock. Depending
  | on the context, one may also require
  | that the samples be independent and
  | have identical probability distribution
  | (in other words independent and identically
  | distributed random variables are the
  | simplest representation of white noise).
  | In particular, if each sample has a normal
  | distribution with zero mean, the signal
  | is said to be additive white Gaussian
  | noise. (from Wikipedia)
  |
  */
pub fn white_noise() -> f64 {

    /*
       If 𝑥 is a sample from a mean 0 and variance
       1 distribution then 𝜎𝑥+𝜇 is a sample with
       mean 𝜇 and variance 𝜎2. So, all you have
       to do is to scale the variable by the
       standard deviation 𝜎 (square root of the
       variance) before adding the mean 𝜇.
       */

    rand11() as f64
}

#[derive(Debug)]
pub struct Signal {
    pub data: A1d::<f64>,
}

impl Signal {

    pub fn white_noise(n: usize) -> Self {

        let mut x = A1d::zeros(n);

        for sample in x.iter_mut() {
            *sample = white_noise();
        }

        Signal { data: x }
    }

    pub fn average_power_time_domain(&self) -> f64 {

        let n = self.data.dim();

        let sum: f64 = self.data.map(|x| x.abs().pow(2)).iter().sum();

        sum / (n as f64)
    }

    pub fn average_power_freq_domain(&self) -> f64 {

        let plan = Plan::new(Operation::Forward, self.data.dim());

        let mut data = self.data.clone();

        dft::transform(data.as_slice_mut().unwrap(), &plan);

        let n = data.dim();
        let sum: f64 = data.map(|x| x.abs().pow(2)).iter().sum();
        sum / (n.pow(2) as f64)
    }
}
