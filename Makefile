.PHONY: all build install clean

all: build

build:
	cargo build --release

install: build
	install -m755 target/release/stacker /usr/local/bin/stacker

clean:
	cargo clean