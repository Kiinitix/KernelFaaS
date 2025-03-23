mod loader;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: kernel-faas <ebpf_function_path>");
        std::process::exit(1);
    }

    let path = &args[1];

    match loader::load_ebpf_program(path) {
        Ok(_) => println!("eBPF function executed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
