## Initialize the database
.PHONY: init
init:
	docker-compose up -d\
	&& sqlx database create\
	&& sqlx migrate run\
	&& cargo build\
	&& docker-compose down
## && cargo install cargo-watch

## Start up the backend
.PHONY: start
start:
	docker-compose up -d\
	&& cargo test\
	&& cargo run

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
