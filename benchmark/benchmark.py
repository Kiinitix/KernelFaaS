import time
import subprocess

def user_space_function():
    a, b = 10, 20
    return a + b

def benchmark_user_space():
    start = time.time()
    for _ in range(1_000_000):
        user_space_function()
    end = time.time()
    return (end - start) * 1e6

def benchmark_ebpf():
    result = subprocess.run(["sudo", "./target/release/ebpf_program"], capture_output=True, text=True)
    return float(result.stdout.strip())

if __name__ == "__main__":
    print("Running benchmark...")

    user_time = benchmark_user_space()
    print(f"User-space execution time: {user_time:.2f} µs")

    ebpf_time = benchmark_ebpf()
    print(f"Kernel-space execution time (eBPF): {ebpf_time:.2f} µs")

    speedup = user_time / ebpf_time
    print(f"eBPF is {speedup:.2f}x faster than user-space execution!")
