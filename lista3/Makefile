
all: build

build:
	cargo build --release
	cp target/release/dial .
	cp target/release/dijkstra .

clean:
	cargo clean
	rm -f dial
	rm -f dijkstra

