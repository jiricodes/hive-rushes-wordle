MAKEOPTIONS=--no-print-directory

.PHONY: game assistant

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

game:
	cargo run -p game -- data/possible_words.txt

assistant:
	cargo run -p assistant -- data/possible_words.txt
