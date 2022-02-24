MAKEOPTIONS=--no-print-directory

DICT?=data/possible_words.txt

.PHONY: game assistant player

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
	cargo run -p game --release -- $(DICT)

assistant:
	cargo run -p assistant --release -- $(DICT)

player:
	cargo run -p player --release -- $(DICT)

game-dbg:
	cargo run -p game -- $(DICT)

assistant-dbg:
	cargo run -p assistant -- $(DICT)

player-dbg:
	cargo run -p player -- $(DICT)
