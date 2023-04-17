crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vlag_new_x87() {
        let vlag = VLag::new_x87();
        assert_eq!(vlag.v[0], 0.0);
        assert_eq!(vlag.v[1], 0.0);
        assert_eq!(vlag.target_v[0], 0.0);
        assert_eq!(vlag.target_v[1], 0.0);
    }

    #[test]
    fn test_vlag_init_x87() {
        let mut vlag = VLag::new_x87();
        vlag.v[0] = 1.0;
        vlag.v[1] = 2.0;
        vlag.target_v[0] = 3.0;
        vlag.target_v[1] = 4.0;
        vlag.init_x87();
        assert_eq!(vlag.v[0], 0.0);
        assert_eq!(vlag.v[1], 0.0);
        assert_eq!(vlag.target_v[0], 0.0);
        assert_eq!(vlag.target_v[1], 0.0);
    }

    #[test]
    fn test_vlag_process() {
        let mut vlag = VLag::new_x87();
        vlag.target_v[0] = 1.0;
        vlag.process();
        assert_eq!(vlag.v[0], 0.004);
    }

    #[test]
    fn test_vlag_new_value() {
        let mut vlag = VLag::new_x87();
        vlag.new_value(1.0);
        assert_eq!(vlag.target_v[0], 1.0);
    }

    #[test]
    fn test_vlag_instantize() {
        let mut vlag = VLag::new_x87();
        vlag.target_v[0] = 1.0;
        vlag.v[0] = 2.0;
        vlag.instantize();
        assert_eq!(vlag.v[0], 1.0);
    }

    #[test]
    fn test_vlag_start_value() {
        let mut vlag = VLag::new_x87();
        vlag.start_value(1.0);
        assert_eq!(vlag.v[0], 1.0);
        assert_eq!(vlag.target_v[0], 1.0);
    }

    #[test]
    fn test_vlag_process2() {
        let mut vlag = VLag::new_x87();
        vlag.target_v[0] = 1.0;
        for _ in 0..100 {
            vlag.process();
        }
        assert!((vlag.v[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vlag_new_value2() {
        let mut vlag = VLag::new_x87();
        vlag.new_value(1.0);
        assert_eq!(vlag.target_v[0], 1.0);
    }

    #[test]
    fn test_vlag_instantize2() {
        let mut vlag = VLag::new_x87();
        vlag.target_v[0] = 1.0;
        vlag.v[0] = 2.0;
        vlag.instantize();
        assert_eq!(vlag.v[0], 1.0);
    }

    #[test]
    fn test_vlag_start_value2() {
        let mut vlag = VLag::new_x87();
        vlag.start_value(1.0);
        assert_eq!(vlag.v[0], 1.0);
        assert_eq!(vlag.target_v[0], 1.0);
    }

    #[test]
    fn test_vlag_init_x87_2() {
        let mut vlag = VLag::new_x87();
        vlag.target_v[0] = 1.0;
        vlag.v[0] = 2.0;
        vlag.init_x87();
        assert_eq!(vlag.v[0], 0.0);
        assert_eq!(vlag.target_v[0], 0.0);
    }
}

