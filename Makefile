test:
	@cargo nextest run

fix:
	@cargo fmt
	@cargo fix --allow-dirty --allow-staged
	@cargo clippy --fix --allow-dirty --allow-staged
	@cargo nextest run

doc:
	@cd tools/schema-tools && cargo run
