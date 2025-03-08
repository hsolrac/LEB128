run: clean format
	@cargo run

clean: 
	@echo "Cleaning build dir"
	@rm -rf target/*
	@cargo clean

check:
	@echo "Checking $(NAME)"
	@cargo check

format: 
	@echo "Format"
	@cargo fmt
