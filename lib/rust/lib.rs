#[inline]
pub fn get_version() -> String {
    return env!("CARGO_PKG_VERSION").to_string();
}
