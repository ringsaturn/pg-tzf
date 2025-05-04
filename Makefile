test:
	cargo pgrx test --release pg13
	cargo pgrx test --release pg14
	cargo pgrx test --release pg15
	cargo pgrx test --release pg16
	cargo pgrx test --release pg17

install:
	cargo pgrx install --release

clean:
	cargo clean

run:
	cargo pgrx run --release

schema:
	cargo pgrx schema pg15 > sql/tzf.sql

package:
	cargo pgrx package

reinstall:
	psql -d postgres -c "DROP EXTENSION IF EXISTS tzf CASCADE;"
	psql -d postgres -c "CREATE EXTENSION tzf;"

basic-examples: reinstall
	psql -d postgres -f examples/query_a_coord.sql
	psql -d postgres -f examples/query_a_batch_coords.sql
	psql -d postgres -f examples/query_a_point.sql
	psql -d postgres -f examples/query_a_batch_points.sql
