prefix ?= /usr/local
bindir = $(prefix)/bin
SYS := $(shell $(CC) -dumpmachine)

build:
	cargo build --release
install: build
ifneq (, $(findstring darwin, $(SYS)))
	install "target/release/wlist" "$(bindir)/wlist"
else
	install -D "target/release/wlist" "$(bindir)/wlist"
endif
uninstall:
	rm -rf "$(bindir)/wlist"
clean:
	rm -rf target
.PHONY: build install uninstall clean