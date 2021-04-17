BUILD_DIR := build

.PHONY: all build clean install package test

all: clean build test

build:
	rustc src/main.rs

clean:
	@echo "meow"

install:
	@echo "meow"

package:
	@echo "meow"

test:
	@echo "meow"