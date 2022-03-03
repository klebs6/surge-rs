#[macro_export] 
macro_rules! m128_mask_signbit          { () => {  _mm_set1_ps(0x80000000 as f32) } }
#[macro_export] macro_rules! m128_mask_absval           { () => {  _mm_set1_ps(0x7fffffff as f32) } }
#[macro_export] macro_rules! m128_zero                  { () => {  _mm_set1_ps(0.0)               } }
#[macro_export] macro_rules! m128_half                  { () => {  _mm_set1_ps(0.5)               } }
#[macro_export] macro_rules! m128_four                  { () => {  _mm_set1_ps(4.0)               } }
#[macro_export] macro_rules! m128_1234                  { () => {  _mm_set_ps(1.0, 2.0, 3.0, 4.0) } }
#[macro_export] macro_rules! m128_0123                  { () => {  _mm_set_ps(0.0, 1.0, 2.0, 3.0) } }
#[macro_export] macro_rules! z128                       { () => {  _mm_setzero_ps()               } }
#[macro_export] macro_rules! m128_nine_two_zero         { () => {  _mm_set1_ps(0.00920833)        } }
#[macro_export] macro_rules! m128_zero_zero_five        { () => {  _mm_set1_ps(0.05)              } }
#[macro_export] macro_rules! m128_eight_seven_six       { () => {  _mm_set1_ps(0.0876)            } }
#[macro_export] macro_rules! m128_one_zero_three        { () => {  _mm_set1_ps(0.0103592)         } }
#[macro_export] macro_rules! m128_one_eight_five        { () => {  _mm_set1_ps(0.185)             } }
#[macro_export] macro_rules! m128_zero_four_five        { () => {  _mm_set1_ps(0.45)              } }
#[macro_export] macro_rules! m128_zero_five             { () => {  _mm_set1_ps(0.5)               } }
#[macro_export] macro_rules! m128_one                   { () => {  _mm_set1_ps(1.0)               } }
#[macro_export] macro_rules! m128_one_three_five        { () => {  _mm_set1_ps(1.035)             } }
#[macro_export] macro_rules! m128_two                   { () => {  _mm_set1_ps(2.0)               } }
#[macro_export] macro_rules! m128_three                 { () => {  _mm_set1_ps(3.0)               } }
#[macro_export] macro_rules! m128_gain_adjustment_2pole { () => {  _mm_set1_ps(0.74)              } }
#[macro_export] macro_rules! m128_gain_adjustment_4pole { () => {  _mm_set1_ps(0.6)               } }

