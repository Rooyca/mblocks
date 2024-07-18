all:
	@echo "Building..."
	cargo build --release
	@echo "Installing..."
	cp target/release/mblocks /home/$(shell basename $(USER))/.cargo/bin
	@echo "Done! :)"