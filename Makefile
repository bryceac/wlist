prefix ?= /usr/local
bindir = $(prefix)/bin
resourcedir = /usr/local/share/wlist
SYS := $(shell $(CC) -dumpmachine)

build:
	cargo build --release
install: build
ifneq (, $(findstring darwin, $(SYS)))
	test ! -d $(resourcedir) && mkdir -p $(resourcedir)

	install "target/release/wlist" "$(bindir)/wlist"
	install "gift_registry.sql" "$(resourcedir)/gift_register.sql"
else
	install -D "target/release/wlist" "$(bindir)/wlist"
	install "gift_registry.sql" "$(resourcedir)/gift_registry.sql"
endif
uninstall:
	rm -rf "$(bindir)/wlist"
	rm -rf "$(resourcedir)"
clean:
	rm -rf target
.PHONY: build install uninstall clean