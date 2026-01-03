fix:
	@cargo fmt
	@cargo fix --allow-dirty --allow-staged
	@cargo clippy --fix --allow-dirty --allow-staged
	@cargo test
