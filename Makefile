SUBDIRS = week1

.PHONY: all format lint test run release rust-version $(SUBDIRS)

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

$(SUBDIRS):
	$(MAKE) -C $@ $(MAKECMDGOALS)

clean format lint test build release all: $(SUBDIRS)
