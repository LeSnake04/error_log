set positional-arguments


default:
	just --list

alias t := test
test:
	cargo test --color always --all-features
	cargo run --package anyhow_example --color always
	cargo run --package async_example --color always

alias b := book
@book cmd:
	cd docs && mdbook $1

book-publish:
	just book build || exit
	scp -r docs/book/html/* webedit@lesnake.xyz:/var/www/html/opt/error_log
