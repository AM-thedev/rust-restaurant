## Initialize the database
.PHONY: init
init:
	cd ../backend && make init

## Start up the backend
.PHONY: start
start:
	cd ../backend && make start

## Shut down the backend
.PHONY: stop
stop:
	cd ../backend && make stop

## Run backend tests
.PHONY: test
test:
	cd ../backend && make test

## Start up the frontend
.PHONY: front
front:
	npm start

## Kill the websocket occupying the frontend address
.PHONY: front-kill
address != lsof -ti :3000
front-kill:
	sudo kill -9 $(address)

## Kill the websocket occupying the backend address
.PHONY: kill
kill:
	cd ../backend && make kill