set positional-arguments


default:
	just --list

alias t := test
test:
	cargo test --color always --all-features
	cargo test --color always
	cargo test --color always --no-default-features --features default_no_std
	cargo test --color always --no-default-features
	cargo run --color always --example anyhow --features anyhow
	cargo run --color always --example async --features anyhow
alias b := book
@book cmd:
	cd docs && mdbook $1

book-publish:
	just book build || exit
	rsync docs/book/html/* webedit@lesnake.xyz:/var/www/html/opt/error_log
