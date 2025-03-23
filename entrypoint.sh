echo "Loading KernelFaaS eBPF function..."

mount | grep /sys/fs/bpf > /dev/null || mount -t bpf bpf /sys/fs/bpf

/app/target/release/kernel-faas /app/ebpf/function.bpf.o
