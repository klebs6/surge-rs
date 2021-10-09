ix!();

pub fn map_2pole_resonance(mut reso: f64, freq: f64, subtype: FilterSubType) -> f64 
{
    match subtype {
        FilterSubType::Medium => {
            reso *= std::cmp::max(FloatOrd(0.0), FloatOrd(1.0 - std::cmp::max(FloatOrd(0.0), FloatOrd((freq - 58.0) * 0.05)).0)).0;
            (0.99 - 1.0 * limit_range((1.0 - (1.0 - reso) * (1.0 - reso)) as f32, 0.0, 1.0)).into()
        },
        FilterSubType::Rough => {
            reso *= std::cmp::max(FloatOrd(0.0), FloatOrd(1.0 - std::cmp::max(FloatOrd(0.0), FloatOrd((freq - 58.0) * 0.05)).0)).0;
            (1.0 - 1.05 * limit_range((1.0 - (1.0 - reso) * (1.0 - reso)) as f32, 0.001, 1.0)).into()
        },
        FilterSubType::Smooth => {
            (2.5 - 2.45 * limit_range((1.0 - (1.0 - reso) * (1.0 - reso)) as f32, 0.0, 1.0)).into()
        },
        _ => 0.0
    }
}

pub fn map_2pole_resonance_noboost(reso: f64, _freq: f64, subtype: FilterSubType) -> f64 
{
    match subtype {
        FilterSubType::Rough => {
            (1.0 - 0.99 * limit_range((1.0 - (1.0 - reso) * (1.0 - reso)) as f32, 0.001, 1.0)).into()
        },
        _ => {
            (0.99 - 0.98 * limit_range((1.0 - (1.0 - reso) * (1.0 - reso)) as f32, 0.0, 1.0)).into()
        },
    }
}

pub fn map_4pole_resonance(mut reso: f64, freq: f64, subtype: FilterSubType) -> f64 
{
    match subtype {
        FilterSubType::Medium => {
            reso *= std::cmp::max(FloatOrd(0.0), FloatOrd(1.0 - std::cmp::max(FloatOrd(0.0), FloatOrd((freq - 58.0) * 0.05)).0)).0;
            // sqrt(1.01) = 1.004987562
            (0.99 - 0.9949 * limit_range(reso as f32, 0.0, 1.0)).into()

        },
        FilterSubType::Rough => {
            reso *= std::cmp::max(FloatOrd(0.0), FloatOrd(1.0 - std::cmp::max(FloatOrd(0.0), FloatOrd((freq - 58.0) * 0.05)).0)).0;
            (1.0 - 1.05 * limit_range(reso as f32, 0.001, 1.0)).into()
        },
        _ => {
            (2.5 - 2.3 * limit_range(reso as f32, 0.0, 1.0)).into()
        }
    }
}

pub fn resoscale(reso: f64, subtype: FilterSubType) -> f64 
{
    match subtype {
        FilterSubType::Medium =>  1.0 - 0.75 * reso * reso,
        FilterSubType::Rough  => 1.0 - 0.5 * reso * reso,
        FilterSubType::Smooth => 1.0 - 0.25 * reso * reso,
        _ => 1.0
    }
}

pub fn resoscale_4pole(reso: f64, subtype: FilterSubType) -> f64 
{
    match subtype {
        FilterSubType::Medium =>  1.0 - 0.75 * reso,
        FilterSubType::Rough  => 1.0 - 0.5 * reso * reso,
        FilterSubType::Smooth => 1.0 - 0.5 * reso,
        _ => 1.0
    }
}

