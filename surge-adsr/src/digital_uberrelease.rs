/// This function is a method of the
/// `AdsrEnvelope` struct in the `surge-adsr` Rust
/// crate. It is called
/// `process_block_digital_uberrelease` and is
/// responsible for calculating the output of the
/// envelope generator in response to a trigger,
/// during the release phase.
/// 
/// Here's a breakdown of what the function does:
/// 
/// 1. The first line `self.phase -=
/// self.tables.envelope_rate_linear(-6.5);`
/// decrements the `phase` variable of the
/// `AdsrEnvelope` struct by the rate of the
/// envelope, which is retrieved from a lookup
/// table based on a constant value of -6.5
/// dB. This corresponds to an exponential rate of
/// decay.
/// 
/// 2. The second line `self.output = self.phase;`
/// assigns the `phase` variable to the `output`
/// variable of the struct. This is the value that
/// will be returned by the envelope generator.
/// 
/// 3. The following `for` loop multiplies the
/// `output` variable by the `phase` variable
/// a number of times, based on the value of
/// a parameter called `ReleaseShape`. The loop
/// iterates for
/// `pvali![self.params[AdsrParam::ReleaseShape]]`
/// times, which is the integer value of the
/// parameter. The purpose of this loop is to
/// shape the release curve of the envelope, which
/// is a polynomial function of the form `y
/// = x^n`.
/// 
/// 4. The `if` statement checks whether the
/// `phase` variable has become negative. If it
/// has, then the envelope generator is in the
/// idle state and the `output` variable is set to
/// 0.0.
/// 
/// 5. The final line `self.output *=
/// self.scalestage;` scales the `output` variable
/// by a value called `scalestage`, which is
/// a constant multiplier used to adjust the
/// overall amplitude of the envelope.
/// 
/// In summary, this function implements the
/// digital release phase of an ADSR envelope
/// generator, which calculates a curve that
/// describes how the amplitude of a sound changes
/// over time. The curve is shaped by a polynomial
/// function based on a user-defined parameter,
/// and is scaled by a constant value.

crate::ix!();

impl AdsrEnvelope {

    pub fn process_block_digital_uberrelease(&mut self) {

        self.phase -= self.tables.envelope_rate_linear(-6.5);

        self.output = self.phase;

        for _i in 0..pvali![self.params[AdsrParam::ReleaseShape]] {
            self.output *= self.phase;
        }

        if self.phase < 0.0
        {
            self.envstate = AdsrState::Idle;
            self.output = 0.0;
        }

        self.output *= self.scalestage;
    }
}
