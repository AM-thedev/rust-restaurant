## Initialize the database
.PHONY: init
init:
	cargo install cargo-watch\
	&& cargo install sqlx-cli --no-default-features --features native-tls,postgres\
	&& sqlx database create\
	&& sqlx migrate run\
	&& cargo test

## Start up the backend
.PHONY: start
start:
	docker-compose up -d\
	&& cargo watch -q -w src/ -x run

## Shut down the backend
.PHONY: stop
stop:
	docker-compose down

## Run tests
.PHONY: test
test:
	cargo test

## Start up the frontend
.PHONY: front
front:
	cd ../frontend && make start

## Kill the websocket occupying the frontend address
.PHONY: front-kill
front-kill:
	cd ../frontend && make front-kill

## Kill the websocket occupying the backend address
.PHONY: kill
address != lsof -ti :8000
kill:
	sudo kill -9 $(address)
