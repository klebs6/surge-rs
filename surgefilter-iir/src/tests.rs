crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    #[test] fn construct_iir_filter() {

        let srunit = SampleRateHandle::default();
        let tuner  = TunerHandle::new(&srunit);
        let tables = TablesHandle::new(&srunit);

        let mut filter = crate::IIRFilter::new_default(&tables,&tuner);
        filter.iirtype =  FilterTypeIIR::HighPass;

        println!("filter: {:?}",filter);
    }
}
