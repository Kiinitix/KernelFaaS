FROM rust:latest

RUN apt-get update && apt-get install -y python3 python3-pip clang llvm libbpf-dev

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["python3", "benchmark.py"]


# docker build -t ebpf-benchmark .
# docker run --privileged ebpf-benchmark


# sudo apt update && sudo apt install -y clang llvm libbpf-dev python3 python3-pip
# cargo build --release
# python3 benchmark.py
