CC = clang
RUSTC = cargo

all: build

build:
	$(CC) -O2 -Wall -shared -o target/release/helper.so src/helper.c -lbpf
	$(RUSTC) build --release

clean:
	rm -rf target/
