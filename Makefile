test:
	cargo pgrx test pg13
	cargo pgrx test pg14
	cargo pgrx test pg15
	cargo pgrx test pg16

install:
	cargo pgrx install

clean:
	cargo clean

run:
	cargo pgrx run

schema:
	cargo pgrx schema > sql/tzf_pg.sql
