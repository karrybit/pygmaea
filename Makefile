fmt:
	docker run -t -w /mnt -v ${PWD}:/mnt monkey cargo fmt

check:
	docker run -t -w /mnt -v ${PWD}:/mnt monkey cargo check

build:
	docker run -t -w /mnt -v ${PWD}:/mnt monkey cargo build

run:
	docker run -t -w /mnt -v ${PWD}:/mnt monkey cargo run

test:
	docker run -t -w /mnt -v ${PWD}:/mnt monkey cargo test

coverage:
	docker run -t -w /mnt -v ${PWD}:/mnt monkey sh coverage.sh

shell:
	docker run -it -w /mnt -v ${PWD}:/mnt monkey bash
