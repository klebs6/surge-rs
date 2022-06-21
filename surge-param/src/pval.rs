crate::ix!();

//TODO: figure out how to collapse this or not need it somehow
//TODO: this will bug if we call on a type which we cannot unwrap into
#[macro_export] macro_rules! pval {
    ($p:expr, $t:ty) => ({
        let result: $t = $p.val.try_into().unwrap();
        result
    });
}
#[macro_export] macro_rules! pvalfref {
    ($p:expr) => ({
        match $p.val {
            PData::Float(ref mut f) => f,
            _ => panic!("parameter is not PData::Float"),
        }
    });
}

#[macro_export] macro_rules! pvaliref {
    ($p:expr) => ({
        match $p.val {
            PData::Int(ref mut v) => v,
            _ => panic!("parameter is not PData::Int"),
        }
    });
}

#[macro_export] macro_rules! pvalbref {
    ($p:expr) => ({
        match $p.val {
            PData::Bool(ref mut v) => v,
            _ => panic!("parameter is not PData::Bool"),
        }
    });
}

#[macro_export] macro_rules! pvalmax {
    ($p:expr, $t:ty) => ({
        let result: $t = $p.delegate.max_value().try_into().unwrap();
        result
    });
}

#[macro_export] macro_rules! pvalmin {
    ($p:expr, $t:ty) => ({
        let result: $t = $p.delegate.min_value().try_into().unwrap();
        result
    });
}

#[macro_export] macro_rules! pvalf { ($p:expr) => ( pval![$p,f32]) }
#[macro_export] macro_rules! pvali { ($p:expr) => ( pval![$p,i32]) }
#[macro_export] macro_rules! pvalb { ($p:expr) => ( pval![$p,bool]) }

#[macro_export] macro_rules! pvalmaxf { ($p:expr) => ( pvalmax![$p,f32]) }
#[macro_export] macro_rules! pvalmaxi { ($p:expr) => ( pvalmax![$p,i32]) }
#[macro_export] macro_rules! pvalmaxb { ($p:expr) => ( pvalmax![$p,bool]) }

#[macro_export] macro_rules! pvalminf { ($p:expr) => ( pvalmin![$p,f32]) }
#[macro_export] macro_rules! pvalmini { ($p:expr) => ( pvalmin![$p,i32]) }
#[macro_export] macro_rules! pvalminb { ($p:expr) => ( pvalmin![$p,bool]) }
