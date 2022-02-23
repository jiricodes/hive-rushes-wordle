MAKEOPTIONS=--no-print-directory

all:
	@make $(MAKEOPTIONS) test

build:
	cargo build --release

test:
	@cargo test

test-verbal:
	@cargo test -- --nocapture

doc:
	cargo doc --no-deps --open