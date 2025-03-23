# **KernelFaaS**

KernelFaaS is a high-performance serverless execution layer that runs functions directly inside the Linux kernel using eBPF. Traditional serverless platforms like AWS Lambda introduce delays due to cold starts, virtualization, and user-space overhead. KernelFaaS eliminates these bottlenecks by executing functions at the kernel level, reducing execution time to microseconds.

This approach is useful for applications that require near-instant response times, such as high-frequency trading, real-time fraud detection, and cybersecurity threat mitigation.

## **How It Works**

Instead of running functions on top of the operating system, KernelFaaS allows functions to be safely executed inside the Linux kernel. This removes the need for context switching and significantly reduces latency. The system ensures stability and security with built-in execution limits, resource isolation, and observability tools.

## **Installation**

### **Prerequisites**

- Linux with eBPF support (Kernel 5.4+ recommended)
- Docker (optional, for containerized deployment)
- Rust (for building the CLI)
- Clang & LLVM (for compiling eBPF programs)

### **Manual Installation**

1. Clone the repository:
   ```sh
   git clone https://github.com/Kiinitix/KernelFaaS 
   cd KernelFaaS
   ```  

2. Install dependencies:
   ```sh
   sudo apt update && sudo apt install clang llvm libelf-dev libbpf-dev build-essential -y
   ```  

3. Build the project:
   ```sh
   cargo build --release
   ```  

4. Run the CLI:
   ```sh
   ./target/release/KernelFaaS run --function example.ebpf
   ```  

### **Docker Installation**

1. Install Docker:
   ```sh
   curl -fsSL https://get.docker.com | sh
   sudo usermod -aG docker $USER
   ```  

2. Build and run the container:
   ```sh
   docker build -t KernelFaaS .
   docker run --privileged --rm KernelFaaS  
   ```  

The `--privileged` flag is required because eBPF interacts with the kernel.

## **Why KernelFaaS?**

Existing serverless platforms rely on user-space execution, which introduces unnecessary delays. KernelFaaS takes advantage of eBPF’s ability to run in the kernel safely, making real-time computing more efficient without compromising security.

Feel free to open issues or contribute by submitting pull requests!  

