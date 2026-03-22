docker-run:
	docker build -t computor-env . && docker run -it -v $(PWD):/home/rustuser/app computor-env

build:
	cargo build --release

test:
	clear && cargo test --no-fail-fast -- --color always --no-capture

run:
	cargo run

clean:
	cargo clean

lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt --check

basic: 
	cargo run -- 'x^2 + 2x^2 + x - x + 1 - 10  = 0'