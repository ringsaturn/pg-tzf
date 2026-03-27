PGRXW := ./scripts/pgrxw.sh
PG_VERSIONS := 14 15 16 17 18
DIST_DIR := dist
EXT_VERSION := $(shell sed -nE 's/^version = "([^"]+)".*/\1/p' Cargo.toml | head -n1)
OS := $(shell uname -s | tr '[:upper:]' '[:lower:]')
ARCH := $(shell uname -m)

.PHONY: test test-all test-pg% install clean run schema package package-all package-pg% clean-dist reinstall basic-examples

test: test-all

test-all:
	@set -e; \
	for pg in $(PG_VERSIONS); do \
		echo "==> Running tests for pg$$pg"; \
		$(PGRXW) test --release pg$$pg; \
	done

test-pg%:
	$(PGRXW) test --release pg$*

install:
	$(PGRXW) install --release

clean:
	cargo clean

clean-dist:
	rm -rf $(DIST_DIR)

run:
	$(PGRXW) run --release

schema:
	$(PGRXW) schema pg16 > sql/tzf.sql

package: package-all

package-all:
	@mkdir -p $(DIST_DIR)
	@set -e; \
	for pg in $(PG_VERSIONS); do \
		$(MAKE) package-pg$$pg; \
	done

package-pg%:
	@set -e; \
	mkdir -p "$(DIST_DIR)"; \
	pg="$*"; \
	pg_config="$$( $(PGRXW) info pg-config "$$pg" )"; \
	out_dir="target/release/tzf-pg$$pg"; \
	echo "==> Packaging pg$$pg with $$pg_config"; \
	$(PGRXW) package --pg-config "$$pg_config" --out-dir "$$out_dir"; \
	so_path="$$(find "$$out_dir" -type f -name 'tzf.so' | head -n1)"; \
	control_path="$$(find "$$out_dir" -type f -name 'tzf.control' | head -n1)"; \
	sql_path="$$(find "$$out_dir" -type f -name 'tzf--*.sql' | head -n1)"; \
	test -n "$$so_path"; \
	test -n "$$control_path"; \
	test -n "$$sql_path"; \
	archive="$(DIST_DIR)/pg-tzf-v$(EXT_VERSION)-pg$$pg-$(OS)-$(ARCH).tar.gz"; \
	tmp_dir="$$(mktemp -d)"; \
	cp "$$so_path" "$$tmp_dir/"; \
	cp "$$control_path" "$$tmp_dir/"; \
	cp "$$sql_path" "$$tmp_dir/"; \
	tar czf "$$archive" -C "$$tmp_dir" .; \
	rm -rf "$$tmp_dir"; \
	echo "Created $$archive"

reinstall:
	psql -d postgres -c "DROP EXTENSION IF EXISTS tzf CASCADE;"
	psql -d postgres -c "CREATE EXTENSION tzf;"

basic-examples: reinstall
	psql -d postgres -f examples/query_a_coord.sql
	psql -d postgres -f examples/query_a_batch_coords.sql
	psql -d postgres -f examples/query_a_point.sql
	psql -d postgres -f examples/query_a_batch_points.sql
