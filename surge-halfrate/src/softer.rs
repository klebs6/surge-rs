ix!();

impl crate::HalfRateFilterSSE {

    pub fn load_softer_coefficients(&mut self, order: usize) {
        match order {
            12 => self.load_softer_rejection150db_tband0_05(),
            10 => self.load_softer_rejection133db_tband0_05(),
            8  => self.load_softer_rejection106db_tband0_05(),
            6  => self.load_softer_rejection80db_tband0_05(),
            4  => self.load_softer_rejection70db_tband0_1(),
            2  => self.load_softer_rejection36db_tband0_1(),
            _  => unreachable!(),
        }
    }

    pub fn load_softer_rejection150db_tband0_05(&mut self) {

        // rejection=150db, transition band=0.05
        let mut a_coefficients: [f64; 6] = [
            0.01677466677723562, 0.13902148819717805,
            0.3325011117394731,  0.53766105314488,
            0.7214184024215805,  0.8821858402078155
        ];

        let mut b_coefficients: [f64; 6] = [
            0.06501319274445962, 0.23094129990840923,
            0.4364942348420355,  0.6329609551399348,
            0.8037808679411123,  0.9599687404800694
        ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

    pub fn load_softer_rejection133db_tband0_05(&mut self) {

        // rejection=133db, transition band=0.05
        let mut a_coefficients: [f64; 5] = [
            0.02366831419883467, 0.18989476227180174,
            0.43157318062118555, 0.6632020224193995, 
            0.860015542499582
        ];

        let mut b_coefficients: [f64; 5] = [
            0.09056555904993387, 0.3078575723749043, 
            0.5516782402507934, 0.7652146863779808, 
            0.9524772837866754
        ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

    pub fn load_softer_rejection106db_tband0_05(&mut self) {

        // rejection=106db, transition band=0.05
        let mut a_coefficients: [f64; 4] = [
            0.03583278843106211, 0.2720401433964576, 
            0.5720571972357003, 0.827124761997324
        ];

        let mut b_coefficients: [f64; 4] = [
            0.1340901419430669, 0.4243248712718685, 
            0.7062921421386394, 0.9415030941737551
        ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

    pub fn load_softer_rejection80db_tband0_05(&mut self) {
        // rejection=80db, transition band=0.05
        let mut a_coefficients: [f64; 3] = [
            0.06029739095712437, 0.4125907203610563, 
            0.7727156537429234
        ];

        let mut b_coefficients: [f64; 3] = [
            0.21597144456092948, 0.6043586264658363, 
            0.9238861386532906
        ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

    pub fn load_softer_rejection70db_tband0_1(&mut self) {

        // rejection=70db,transition band=0.1
        let mut a_coefficients: [f64; 2] = [
            0.07986642623635751, 0.5453536510711322
        ];

        let mut b_coefficients: [f64; 2] = [
            0.28382934487410993, 0.8344118914807379
        ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

    pub fn load_softer_rejection36db_tband0_1(&mut self) {

        // order=2, rejection=36db, transition band=0.1
        let mut a_coefficients: [f64; 1] = [
            0.23647102099689224
        ];
        let mut b_coefficients: [f64; 1] = [
            0.7145421497126001
        ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }
}
