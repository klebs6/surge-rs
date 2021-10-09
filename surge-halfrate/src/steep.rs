ix!();

impl crate::HalfRateFilterSSE {

    pub fn load_steep_coefficients(&mut self, order: usize) {
        match order {
            12 => self.load_steep_rejection104db_tband0_01(),
            10 => self.load_steep_rejection86db_tband0_01(),
            8  => self.load_steep_rejection69db_tband0_01(),
            6 => self.load_steep_rejection51db_tband0_01(),
            4 => self.load_steep_rejection53db_tband0_05(),
            2 => self.load_steep_rejection36db_tband0_1(),
            _ => unreachable!(),
        }
    }

    pub fn load_steep_rejection104db_tband0_01(&mut self) {
        // rejection=104db, transition band=0.01
        let mut a_coefficients: [f64; 6] = 
            [
            0.036681502163648017, 
            0.2746317593794541,
            0.5610989697879195,  
            0.769741833862266,
            0.8922608180038789,   
            0.962094548378084
            ];

        let mut b_coefficients: [f64; 6] = [
            0.13654762463195771, 
            0.42313861743656667,
            0.6775400499741616,  
            0.839889624849638,
            0.9315419599631839,  
            0.9878163707328971
        ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

    pub fn load_steep_rejection86db_tband0_01(&mut self) {
        // rejection=86db, transition band=0.01
        let mut a_coefficients: [f64; 5] = [
            0.051457617441190984, 
            0.35978656070567017,
            0.6725475931034693, 
            0.8590884928249939, 
            0.9540209867860787
        ];

        let mut b_coefficients: [f64; 5] = [
            0.18621906251989334, 
            0.529951372847964, 
            0.7810257527489514, 
            0.9141815687605308, 
            0.985475023014907
        ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

    pub fn load_steep_rejection69db_tband0_01(&mut self) {
        // rejection=69db, transition band=0.01
        let mut a_coefficients: [f64; 4] = [
            0.07711507983241622, 
            0.4820706250610472, 
            0.7968204713315797,  
            0.9412514277740471
        ];

        let mut b_coefficients: [f64; 4] = [
            0.2659685265210946, 
            0.6651041532634957, 
            0.8841015085506159, 
            0.9820054141886075
        ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

    pub fn load_steep_rejection51db_tband0_01(&mut self) {
        // rejection=51db, transition band=0.01
        let mut a_coefficients: [f64; 3] = [
            0.1271414136264853, 
            0.6528245886369117, 
            0.9176942834328115
        ];

        let mut b_coefficients: [f64; 3] = [
            0.40056789819445626, 
            0.8204163891923343, 
            0.9763114515836773
        ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

    pub fn load_steep_rejection53db_tband0_05(&mut self) {
        // rejection=53db,transition band=0.05
        let mut a_coefficients: [f64; 2] = [
            0.12073211751675449, 
            0.6632020224193995
        ];

        let mut b_coefficients: [f64; 2] = [
            0.3903621872345006, 
            0.890786832653497
        ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

    pub fn load_steep_rejection36db_tband0_1(&mut self) {
        // order=2, rejection=36db, transition band=0.1
        let mut a_coefficients: [f64; 1] = [ 0.23647102099689224 ];
        let mut b_coefficients: [f64; 1] = [ 0.7145421497126001  ];

        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

}
