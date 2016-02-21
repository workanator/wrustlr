# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# BUILD TARGETS
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

all: lib \
	 mod \
	 bin

lib: lib-types \
	 lib-conf \
	 lib-io \
	 lib-async \
	 lib-module \
	 lib-log \
	 lib-core

mod: mod-echo

bin: bin-server

run: bin-server
	@mkdir -p bin;
	@cp src/bin/server/target/debug/wrust-server bin/;
	bin/wrust-server;

clean:
	@rm -fr `find . -type d -name target`;
	@rm -fr bin;


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# UPDATE TARGETS
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

update: update-lib \
		update-mod \
		update-bin

update-lib: update-lib-types \
			update-lib-conf \
			update-lib-io \
			update-lib-async \
			update-lib-module \
			update-lib-log \
			update-lib-core

update-mod: update-mod-echo

update-bin: update-bin-server

# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# TEST TARGETS
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

test: test-lib \
	  test-mod

test-lib: test-lib-types \
		  test-lib-conf \
		  test-lib-io \
		  test-lib-async \
		  test-lib-module \
		  test-lib-log \
		  test-lib-core

test-mod: test-mod-echo


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# LIB \ TYPES
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lib-types:
	cargo build --manifest-path="src/lib/types/Cargo.toml";

update-lib-types:
	cargo update --manifest-path="src/lib/types/Cargo.toml";

test-lib-types:
	cargo test --manifest-path="src/lib/types/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# LIB \ CONF
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lib-conf:
	cargo build --manifest-path="src/lib/conf/Cargo.toml";

update-lib-conf:
	cargo update --manifest-path="src/lib/conf/Cargo.toml";

test-lib-conf:
	cargo test --manifest-path="src/lib/conf/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# LIB \ IO
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lib-io:
	cargo build --manifest-path="src/lib/io/Cargo.toml";

update-lib-io:
	cargo update --manifest-path="src/lib/io/Cargo.toml";

test-lib-io:
	cargo test --manifest-path="src/lib/io/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# LIB \ ASYNC
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lib-async:
	cargo build --manifest-path="src/lib/async/Cargo.toml";

update-lib-async:
	cargo update --manifest-path="src/lib/async/Cargo.toml";

test-lib-async:
	cargo test --manifest-path="src/lib/async/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# LIB \ MODULE
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lib-module:
	cargo build --manifest-path="src/lib/module/Cargo.toml";

update-lib-module:
	cargo update --manifest-path="src/lib/module/Cargo.toml";

test-lib-module:
	cargo test --manifest-path="src/lib/module/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# LIB \ LOG
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lib-log:
	cargo build --manifest-path="src/lib/log/Cargo.toml";

update-lib-log:
	cargo update --manifest-path="src/lib/log/Cargo.toml";

test-lib-log:
	cargo test --manifest-path="src/lib/log/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# LIB \ CORE
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lib-core:
	cargo build --manifest-path="src/lib/core/Cargo.toml";

update-lib-core:
	cargo update --manifest-path="src/lib/core/Cargo.toml";

test-lib-core:
	cargo test --manifest-path="src/lib/core/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# MOD \ ECHO
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

mod-echo:
	cargo build --manifest-path="src/mod/echo/Cargo.toml";

update-mod-echo:
	cargo update --manifest-path="src/mod/echo/Cargo.toml";

test-mod-echo:
	cargo test --manifest-path="src/mod/echo/Cargo.toml";


# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# BIN \ SERVER
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

bin-server:
	cargo build --manifest-path="src/bin/server/Cargo.toml";

update-bin-server:
	cargo update --manifest-path="src/bin/server/Cargo.toml";

test-bin-server:
	cargo test --manifest-path="src/bin/server/Cargo.toml";
