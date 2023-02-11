format:
	find {{invocation_directory()}} -name \*.rs -exec rustfmt {} \;

fix:
	cargo fix --allow-dirty

lint:
	just format
	just fix

lint-watch:
	cargo watch -s 'just lint'

test:
	cargo test

test-watch:
	cargo watch -x 'test'
