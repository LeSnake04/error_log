test:
	cargo test --color always
	cargo run --package anyhow_example
t: 
	just test
