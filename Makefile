DOCKER = docker run -it -w /mnt -v ${PWD}:/mnt monkey

check:
	cargo fmt
	$(DOCKER) cargo $@

build:
	cargo fmt
	$(DOCKER) cargo $@

run:
	cargo fmt
	$(DOCKER) cargo $@

test:
	cargo fmt
	$(DOCKER) cargo $@

coverage:
	$(DOCKER) sh coverage.sh

bash:
	$(DOCKER) $@
