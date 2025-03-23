#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <linux/sched.h>

SEC("tracepoint/syscalls/sys_enter_execve")
int bpf_function(struct trace_event_raw_sys_enter *ctx) {
    int pid = bpf_get_current_pid_tgid() >> 32;
    bpf_printk("KernelFaaS: Function executed by PID %d\n", pid);

    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    if (task->real_cred->uid.val != 0) {
        bpf_printk("KernelFaaS: Unauthorized user detected!\n");
        return -1;
    }

    return 0;
}

char _license[] SEC("license") = "GPL";
