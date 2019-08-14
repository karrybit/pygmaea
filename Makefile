fmt:
	docker run -t monkey cargo fmt

check:
	docker run -t monkey cargo check

build:
	docker run -t monkey cargo build

run:
	docker run -t monkey cargo run

test:
	docker run -t monkey cargo test

shell:
	docker run -it -w /mnt -v ${PWD}:/mnt monkey bash
