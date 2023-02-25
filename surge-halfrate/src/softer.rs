crate::ix!();

/// This is a Rust code for a Half Rate Filter
/// that implements several methods to load
/// different sets of coefficients used in the
/// filter. 
///
/// The different `load_softer_rejection` methods
/// are used to load the filter coefficients for
/// different orders, which correspond to
/// different levels of frequency rejection and
/// transition band. The filter coefficients are
/// represented as arrays of `f64` values for the
/// `a` and `b` coefficients, which are then
/// passed to the `store_coefficients` method via
/// pointers to the arrays. 
/// 
/// It's worth noting that the filter coefficients
/// are loaded into mutable arrays that are local
/// to each `load_softer_rejection` method. This
/// ensures that the coefficients are not shared
/// between different filter instances, which
/// could potentially cause unexpected behavior.
/// 
/// Overall, this code provides a way to configure
/// a half-rate filter with different levels of
/// frequency rejection and transition band.
///
impl crate::HalfRateFilterSSE {

    /// The `HalfRateFilterSSE` struct appears to be
    /// a filter that operates on audio data. 
    /// 
    /// The `load_softer_coefficients` method loads
    /// the filter coefficients based on the `order`
    /// parameter passed to it. 
    /// 
    /// The method uses a `match` statement to select
    /// the appropriate filter coefficients based on
    /// the `order` value.
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

    /// This method loads a set of coefficients
    /// with a
    /// 150 dB rejection and a transition band of
    /// 0.05.
    ///
    /// The method mutates the struct by storing
    /// the coefficients.
    ///
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

        // This is an unsafe block that allows us to use pointers
        // to access and modify the arrays directly.
        unsafe {
            self.store_coefficients(
                a_coefficients.as_mut_ptr(), 
                b_coefficients.as_mut_ptr());
        }
    }

    /// The next five methods are similar to the
    /// previous one.
    ///
    /// They load different sets of coefficients
    /// with varying rejections and transition
    /// bands.
    ///
    /// this method has the following
    /// configuration:
    ///
    /// rejection=133db, transition band=0.05
    ///
    pub fn load_softer_rejection133db_tband0_05(&mut self) {

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

    /// rejection=106db, transition band=0.05
    ///
    pub fn load_softer_rejection106db_tband0_05(&mut self) {

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

    /// rejection=80db, transition band=0.05
    ///
    pub fn load_softer_rejection80db_tband0_05(&mut self) {

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

    /// rejection=70db,transition band=0.1
    ///
    pub fn load_softer_rejection70db_tband0_1(&mut self) {

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

    /// order=2, rejection=36db, transition band=0.1
    ///
    pub fn load_softer_rejection36db_tband0_1(&mut self) {

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
