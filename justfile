PREFIX := "/usr/local"
BINDIR := PREFIX+"/bin"
MANDIR := PREFIX+"/share/man"

build:
	cargo build --release
install:
	install "target/release/wlist" "{{BINDIR}}/wlist"
	install "man/wlist.1" "{{MANDIR}}/man1"
	install "man/wlist-add.1" "{{MANDIR}}/man1"
	install "man/wlist-delete.1" "{{MANDIR}}/man1"
	install "man/wlist-export.1" "{{MANDIR}}/man1"
	install "man/wlist-import.1" "{{MANDIR}}/man1"
	install "man/wlist-show.1" "{{MANDIR}}/man1"
	install "man/wlist-update.1" "{{MANDIR}}/man1"
uninstall:
	rm "{{BINDIR}}/wlist"
	rm "{{MANDIR}}/man1/wlist.1"
	rm "{{MANDIR}}/man1/wlist-add.1"
	rm "{{MANDIR}}/man1/wlist-delete.1"
	rm "{{MANDIR}}/man1/wlist-export.1"
	rm "{{MANDIR}}/man1/wlist-import.1"
	rm "{{MANDIR}}/man1/wlist-show.1"
	rm "{{MANDIR}}/man1/wlist-update.1"
clean:
	rm -rf target