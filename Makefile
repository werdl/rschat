cargo:
	cargo build --release
sql:
	sqlite3 database.db < setup.sql

setup: | cargo sql
run:
	cargo run --release
clean:
	rm -rf database.db target/* Cargo.lock