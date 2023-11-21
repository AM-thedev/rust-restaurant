## Initialize the database
.PHONY: init
init:
	cd backend && make init

## Start up the backend
.PHONY: start
start:
	cd backend && make start

## Shut down the backend
.PHONY: stop
stop:
	cd backend && make stop

## Run backend tests
.PHONY: test
test:
	cd backend && make test

## Start up the frontend
.PHONY: front
front:
	cd frontend && npm start

## Kill the websocket occupying the backend address
.PHONY: kill
kill:
	cd backend && make kill