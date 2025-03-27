use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bpf_target = out_dir.join("program.o");

    let bpf_source = "../bpf/program.bpf.c";

    let status = Command::new("clang")
        .args(&[
            "-g",
            "-O2",
            "-target",
            "bpf",
            "-D__KERNEL__",
            "-D__BPF_TRACING__",
            "-I/usr/include/",
            "-I/usr/include/bpf/",
            "-c",
            bpf_source,
            "-o",
        ])
        .arg(bpf_target.to_str().unwrap())
        .status()
        .expect("Failed to compile eBPF program");

    if !status.success() {
        panic!("eBPF compilation failed!");
    }

    println!("cargo:rerun-if-changed={}", bpf_source);
}
