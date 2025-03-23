FROM debian:latest

RUN apt update && apt install -y \
    clang llvm libelf-dev libbpf-dev \
    gcc-multilib make cargo \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

RUN make build

ENTRYPOINT ["/bin/bash", "/app/entrypoint.sh"]

# docker build -t kernelfaas .

# docker run --rm --privileged -v /sys/fs/bpf:/sys/fs/bpf -v /lib/modules:/lib/modules kernelfaas

