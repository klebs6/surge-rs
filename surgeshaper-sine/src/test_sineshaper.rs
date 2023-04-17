crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    #[test] fn smoke() {

        let srunit = SampleRateHandle::default();
        let tables = TablesHandle::new(&srunit);

        let shaper = SineShaper::new(&tables);

        let result = shaper.shape(simd_m128::one(), simd_m128::half());
        println!("result: {:?}", result);

        let result = shaper.shape(result, simd_m128::half());
        println!("result: {:?}", result);
    }
}
