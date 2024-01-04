test:
	cargo test

run:
	cargo run

run-dependencies:
	docker run -d -p 8086:8086 -v influxdb:/var/lib/influxdb influxdb
	docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=postgres -v postgres:/var/lib/postgresql/data postgres
