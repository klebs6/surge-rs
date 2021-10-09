ix!();

#[cfg(target_arch = "x86_64")]
pub fn sign(x: i32) -> i32 
{
    match x < 0 { 
        true  => -1,
        false => 1,
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn sign(x: i32) -> i32 
{
    todo!("extract result");
    unsafe {
        asm![
            "mov eax, 1",
            "mov edx, -1",
            "cmp x, 0",
            "cmovle eax, edx",
        ]
    }
}
