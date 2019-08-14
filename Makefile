fmt:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt monkey cargo fmt

check:
	make fmt
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt monkey cargo check

build:
	make fmt
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt monkey cargo build

run:
	make fmt
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt monkey cargo run

test:
	make fmt
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt monkey cargo test

shell:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt monkey bash
