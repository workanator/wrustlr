# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# BUILD TARGETS
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

all: lib mod bin

lib: lib-types lib-io lib-core

mod: mod-echo

bin: bin-server

run: bin-server
	@mkdir -p bin;
	@cp src/bin/server/target/debug/wrust-server bin/;
	bin/wrust-server;

update: update-lib-types \
		update-lib-io \
		update-lib-core \
		update-mod-echo \
		update-bin-server

clean:
	@rm -fr `find . -type d -name target`;
	@rm -fr bin;


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# TEST TARGETS
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

test-all: test-lib test-mod

test-lib: test-lib-types test-lib-io test-lib-core

test-mod: test-mod-echo


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# LIB TYPES
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lib-types:
	cargo build --manifest-path="src/lib/types/Cargo.toml";

update-lib-types:
	cargo update --manifest-path="src/lib/types/Cargo.toml";

test-lib-types:
	cargo test --manifest-path="src/lib/types/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# LIB IO
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lib-io:
	cargo build --manifest-path="src/lib/io/Cargo.toml";

update-lib-io:
	cargo update --manifest-path="src/lib/io/Cargo.toml";

test-lib-io:
	cargo test --manifest-path="src/lib/io/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# LIB CORE
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lib-core:
	cargo build --manifest-path="src/lib/core/Cargo.toml";

update-lib-core:
	cargo update --manifest-path="src/lib/core/Cargo.toml";

test-lib-core:
	cargo test --manifest-path="src/lib/core/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# MOD ECHO
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

mod-echo:
	cargo build --manifest-path="src/mod/echo/Cargo.toml";

update-mod-echo:
	cargo update --manifest-path="src/mod/echo/Cargo.toml";

test-mod-echo:
	cargo test --manifest-path="src/mod/echo/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# BIN SERVER
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

bin-server:
	cargo build --manifest-path="src/bin/server/Cargo.toml";

update-bin-server:
	cargo update --manifest-path="src/bin/server/Cargo.toml";

test-bin-server:
	cargo test --manifest-path="src/bin/server/Cargo.toml";
