DOCKER = docker run -it -w /mnt -v ${PWD}:/mnt monkey

check:
	cargo fmt
	$(DOCKER) cargo $@

.PHONY: build
build:
	cargo fmt
	$(DOCKER) cargo $@

run:
	cargo fmt
	$(DOCKER) cargo $@

debug: build
	cargo fmt
	$(DOCKER) rust-lldb target/debug/monkey

test:
	cargo fmt
	$(DOCKER) cargo $@

coverage:
	$(DOCKER) sh coverage.sh

bash:
	$(DOCKER) $@
