test:
	cargo test

run:
	cargo run

run-dependencies:
	docker run -d -p 8086:8086 -v influxdb:/var/lib/influxdb influxdb
