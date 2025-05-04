test:
	cargo pgrx test pg13
	cargo pgrx test pg14
	cargo pgrx test pg15
	cargo pgrx test pg16
	cargo pgrx test pg17

install:
	cargo pgrx install

clean:
	cargo clean

run:
	cargo pgrx run

schema:
	cargo pgrx schema > sql/tzf.sql

reinstall:
	psql -d postgres -c "DROP EXTENSION IF EXISTS tzf CASCADE;"
	psql -d postgres -c "CREATE EXTENSION tzf;"

basic-examples: reinstall
	psql -d postgres -f examples/query_a_coord.sql
	psql -d postgres -f examples/query_a_batch_coords.sql
	psql -d postgres -f examples/query_a_point.sql
	psql -d postgres -f examples/query_a_batch_points.sql
