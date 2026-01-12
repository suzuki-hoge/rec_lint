.PHONY: test fix schema doc fix-all install

test:
	@cargo nextest run

fix:
	@cargo fmt
	@cargo fix --allow-dirty --allow-staged
	@cargo clippy --fix --allow-dirty --allow-staged
	@cargo nextest run
	@rec_lint validate src
	@rec_lint validate tests

gen:
	@cargo run -q --manifest-path tools/schema-tools/Cargo.toml -- bundle
	@cargo run -q --manifest-path tools/schema-tools/Cargo.toml -- doc
	@cargo run -q --manifest-path tools/readme-generator/Cargo.toml

fix-all:
	@make fix
	@make gen

install:
	@cargo build --release
	@cargo install --path .
