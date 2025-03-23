extern "C" {
    fn load_ebpf_function(path: *const i8) -> i32;
}

pub fn load_ebpf_program(path: &str) -> Result<(), String> {
    let c_path = std::ffi::CString::new(path).unwrap();
    let result = unsafe { load_ebpf_function(c_path.as_ptr()) };

    if result != 0 {
        return Err("Failed to load eBPF function".to_string());
    }
    Ok(())
}
