PGRXW=./scripts/pgrxw.sh

test:
	$(PGRXW) test --release pg14
	$(PGRXW) test --release pg15
	$(PGRXW) test --release pg16
	$(PGRXW) test --release pg17
	$(PGRXW) test --release pg18

install:
	$(PGRXW) install --release

clean:
	cargo clean

run:
	$(PGRXW) run --release

schema:
	$(PGRXW) schema pg16 > sql/tzf.sql

package:
	$(PGRXW) package

reinstall:
	psql -d postgres -c "DROP EXTENSION IF EXISTS tzf CASCADE;"
	psql -d postgres -c "CREATE EXTENSION tzf;"

basic-examples: reinstall
	psql -d postgres -f examples/query_a_coord.sql
	psql -d postgres -f examples/query_a_batch_coords.sql
	psql -d postgres -f examples/query_a_point.sql
	psql -d postgres -f examples/query_a_batch_points.sql
