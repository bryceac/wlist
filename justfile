PREFIX := "/usr/local"
BINDIR := PREFIX+"/bin"

build:
	cargo build --release
install:
    install "target/release/wlist" "{{BINDIR}}/wlist"
uninstall:
	rm "$(bindir)/wlist"
clean:
	rm -rf target