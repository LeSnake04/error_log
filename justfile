test:
	cargo test --color always
	cargo run --package anyhow_example --color always
t: 
	just test
