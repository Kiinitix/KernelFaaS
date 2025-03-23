# eBPF Benchmark: User-Space vs. Kernel-Space Execution  

## Overview  
This project compares the execution speed of running a function in **user space** vs. running the same function in **kernel space using eBPF**. It demonstrates how **eBPF eliminates user-space overhead** and significantly speeds up function execution inside the Linux kernel.  

### Why Does This Matter?  
Traditional function execution involves user-space transitions and system call overhead, which slow things down. With **eBPF**, we can execute lightweight functions inside the kernel, avoiding unnecessary context switches and achieving near-zero latency.  

---  

## Getting Started  

### Option 1: Run with Docker (Recommended)  
This is the easiest way to run the benchmark without installing dependencies.  

```bash  
# Clone the repository  
git clone https://github.com/Kiinitix/KernelFaaS  
cd KernelFaaS/benchmark  

# Build and run the Docker container  
docker build -t ebpf-benchmark .  
docker run --privileged ebpf-benchmark  
```  

---  

### Option 2: Run Manually (Without Docker)  
If you prefer running the benchmark directly on your system, follow these steps:  

#### 1. Install Dependencies  
Ensure your system has the necessary tools:  

```bash  
sudo apt update  
sudo apt install -y clang llvm libbpf-dev python3 python3-pip  
```  

#### 2. Build the eBPF Program  
Compile the Rust-based eBPF function:  

```bash  
cargo build --release  
```  

#### 3. Run the Benchmark  
Execute the Python benchmarking script:  

```bash  
python3 benchmark.py  
```  

---  

## Benchmark Results  
The script runs the same function **1,000,000 times** in both user space and kernel space (eBPF) and compares execution time.  

**Expected Output:**  
```bash  
User-space execution time: 500.12 µs  
Kernel-space execution time (eBPF): 12.34 µs  
eBPF is 40.52x faster than user-space execution!  
```  

The exact numbers will vary based on hardware, but **eBPF execution should be significantly faster** than user-space execution.

---  

## Real-World Applications  
This method can be applied in various high-performance computing scenarios, including:  
- **High-Frequency Trading** – Execute strategies with near-instant speed.  
- **Real-Time Security Monitoring** – Detect system anomalies efficiently.  
- **Cloud Computing** – Reduce serverless function cold starts to zero.  

---  

## Contributing  
Feel free to open issues or contribute by submitting pull requests!  

