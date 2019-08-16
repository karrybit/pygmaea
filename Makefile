DOCKER = docker run -it -w /mnt -v ${PWD}:/mnt monkey

fmt:
	$(DOCKER) cargo $@

check:
	$(DOCKER) cargo $@

build:
	$(DOCKER) cargo $@

run:
	$(DOCKER) cargo $@

test:
	$(DOCKER) cargo $@

coverage:
	$(DOCKER) sh coverage.sh

bash:
	$(DOCKER) $@
