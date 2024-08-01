prog :=xnixperms

build:
	cargo build $(release)

install:
	cp target/$(target)/$(prog) ~/bin/$(prog)-$(extension)

all: build install
 
help:
	@echo "usage: make $(prog)"