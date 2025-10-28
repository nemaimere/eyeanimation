
SLOT=$(shell cat slot.txt)
USER=node
TAG=claude-eyeanimation
FULLTAG=$(TAG)$(SLOT)

# containercmd=docker
CONTAINERCMD=podman

build:
	$(CONTAINERCMD) build -t $(FULLTAG) .

run:
	$(CONTAINERCMD) run -it --rm -u root \
		-v `pwd`/home:/root \
		-v `pwd`/..:/workspace \
		$(FULLTAG) bash

#		-v `pwd`/home:/home/node \

root:
	$(MAKE) USER=root run

run_old:
	$(CONTAINERCMD) run -it --rm \
		-v `pwd`/claude-persist/.claude:/home/node/.claude \
		-v `pwd`/claude-persist/.claude.json:/home/node/.claude.json \
		claude-$(SLOT) bash
