// Minimal ESP32 support library
#[doc(hidden)]
pub fn link() {
    extern "C" {
        fn esp_app_get_elf_sha256_256(sha_256: *const u32) -> i32;
    }
    unsafe {
        let _ = esp_app_get_elf_sha256_256(core::ptr::null());
    }
}
