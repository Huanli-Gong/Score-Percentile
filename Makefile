rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

watch:
	cargo lambda watch

build: 
	cargo lambda build --release 

deploy:
	cargo lambda deploy 

aws-invoke: 
	cargo lambda invoke --remote score_percentile --data-ascii '{"score": 99}'

invoke:
	cargo lambda invoke --data-ascii '{"score": 99}'

all: format lint test 