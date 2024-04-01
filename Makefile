SUBDIRS = week1

.PHONY: all format lint test run release rust-version

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter


$(SUBDIRS):
    $(MAKE) -C $@ $(MAKECMDGOALS)

clean: $(SUBDIRS)

format: $(SUBDIRS)

lint: $(SUBDIRS)

test: $(SUBDIRS)

builD: $(SUBDIRS)

release: $(SUBDIRS)

all: format lint test release
