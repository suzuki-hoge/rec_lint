test:
	@cargo nextest run

fix:
	@cargo fmt
	@cargo fix --allow-dirty --allow-staged
	@cargo clippy --fix --allow-dirty --allow-staged
	@cargo nextest run

doc:
	@cargo run -q --manifest-path tools/schema-tools/Cargo.toml
	@cargo run -q --manifest-path tools/readme-generator/Cargo.toml

fix-all:
	@make fix
	@make doc

install:
	@cargo build --release
	@cargo install --path .
