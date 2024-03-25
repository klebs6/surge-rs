#[test] fn test_log() {
    {
        let a: f32 = 1.5_f32.log(10.0) / 2.0_f32.log(10.0);
        let b: f32 = 1.5_f32.log2();
        assert_eq!(a,b);
    }
    { 
        let a: f64 = 1.333333333333333333_f64.log(10.0) / 2.0_f64.log(10.0);
        let b: f64 = 1.333333333333333333_f64.log2();
        assert_eq!(a,b);
    }
}

