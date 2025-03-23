#include <bpf/libbpf.h>
#include <stdio.h>
#include <stdlib.h>

int load_ebpf_function(const char *path) {
    struct bpf_object *obj;
    int prog_fd;

    obj = bpf_object__open_file(path, NULL);
    if (!obj) {
        fprintf(stderr, "Failed to open eBPF object\n");
        return -1;
    }

    if (bpf_object__load(obj)) {
        fprintf(stderr, "Failed to load eBPF program\n");
        return -1;
    }

    prog_fd = bpf_program__fd(bpf_object__find_program_by_name(obj, "bpf_function"));
    if (prog_fd < 0) {
        fprintf(stderr, "Failed to attach eBPF program\n");
        return -1;
    }

    printf("✅ eBPF function loaded successfully!\n");
    return 0;
}
