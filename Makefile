## Initialize the database
.PHONY: init
init:
	cd backend && make init

## Start up the backend
.PHONY: start
start:
	cd backend && make start

## Start up the backend in dev mode
.PHONY: start-dev
start:
	cd backend && make start-dev

## Shut down the backend
.PHONY: stop
stop:
	cd backend && make stop