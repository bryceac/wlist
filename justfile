PREFIX := "/usr/local"
BINDIR := PREFIX+"/bin"

build:
	cargo build --release
install:
    install "target/release/wlist" "{{BINDIR}}/wlist"
uninstall:
	rm "{{BINDIR}}}}/wlist"
clean:
	rm -rf target