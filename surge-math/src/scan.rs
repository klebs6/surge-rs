crate::ix!();

pub fn bitscan_reverse(bits: u32) -> u32 
{
    core::intrinsics::cttz(bits)
}
