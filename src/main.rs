
#[unsafe(no_mangle)]
unsafe extern "Rust" fn __getrandom_v03_custom(
    dest: *mut u8,
    len: usize,
) -> Result<(), getrandom_03::Error> {
    if dest.is_null() {
        return Err(getrandom_03::Error::UNSUPPORTED);
    }

    let slice = unsafe { std::slice::from_raw_parts_mut(dest, len) };
    fill_random_bytes(slice);
    Ok(())
}

pub fn always_fail_02(buf: &mut [u8]) -> Result<(), getrandom_02::Error> {
    fill_random_bytes(buf);
    Ok(())
}

getrandom_02::register_custom_getrandom!(always_fail_02);


fn main() {
    println!("Hello, world!");
}
