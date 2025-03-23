use aya::programs::TracePoint;
use aya::Bpf;
use std::time::Instant;
use std::fs;

fn main() {
    let mut bpf = Bpf::load_file("ebpf_program.bpf.o").unwrap();
    let prog: &mut TracePoint = bpf.program_mut("benchmark_ebpf").unwrap().try_into().unwrap();
    prog.load().unwrap();
    prog.attach("syscalls", "sys_enter_write").unwrap();

    let start = Instant::now();
    for _ in 0..1_000_000 {
        unsafe { libc::syscall(libc::SYS_gettid); }
    }
    let elapsed = start.elapsed().as_micros();
    
    println!("{}", elapsed);
}
