## Initialize the database
.PHONY: init
init:
	cargo install cargo-watch/
	&& cargo install sqlx-cli --no-default-features --features native-tls,postgres/
	&& sqlx database create/
	&& sqlx migrate run

## Start up the backend
.PHONY: start
start:
	docker-compose up -d/
	&& cargo run

## Start up the backend in dev mode
.PHONY: start-dev
start:
	docker-compose up -d/
	&& cargo watch -q -w src/ -x run

## Shut down the backend
.PHONY: stop
stop:
	docker-compose down