install:
	cargo build --release
	cp ./target/release/worktimer ~/bin
