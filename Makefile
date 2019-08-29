DOCKER = docker run -it -w /mnt -v ${PWD}:/mnt monkey

.PHONY: fmt
fmt:
	$(DOCKER) cargo $@

.PHONY: clippy
clippy:
	$(DOCKER) cargo $@

check: fmt clippy
	$(DOCKER) cargo $@

.PHONY: build
build: fmt clippy
	$(DOCKER) cargo $@

run: fmt clippy
	$(DOCKER) cargo $@

debug: fmt clippy build
	$(DOCKER) rust-lldb target/debug/monkey

test: fmt clippy
	$(DOCKER) cargo $@

coverage:
	$(DOCKER) sh coverage.sh

bash:
	$(DOCKER) $@
