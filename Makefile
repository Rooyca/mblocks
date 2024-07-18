all:
	@echo "Building..."
	cargo build --release
	@echo "Installing..."
	cargo install --path .
	@echo "Done! :)"