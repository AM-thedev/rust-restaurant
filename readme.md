<!-- ABOUT THE PROJECT -->
### Rust Restaurant API
## About The Project

A simple Rust api for retrieving, adding, and deleting restaurant orders from a table.
Made using Axum and PostgreSQL, a pre-filled .env file is included for convenience.
When the backend starts, 5 fake users will start making various requests to the server at random intervals.


<!-- GETTING STARTED -->
## Getting Started

If you want to test the api manually, simply import the `Rust Restaurant Api.postman.json` file into Postman.
Execute the following to get the api up and running:

### Instructions

1. Clone the repo
   ```sh
   git clone https://github.com/AM-thedev/rust-restaurant.git
   ```
2. Enter the project folder
   ```sh
   cd rust-restaurant
   ```
3. Initialize the server
   ```sh
   make init
   ```
4. Start up the backend at: `localhost:8000`
   ```sh
   make start
   ```

### Additional Commands

* Shut down the backend
   ```sh
   make stop
   ```
* Start up the frontend, access at: `localhost:3000`
   ```sh
   make start-front
   ```
* Shut down the frontend
   ```sh
   make stop-front
   ```
* Run backend tests
   ```sh
   make test
   ```
* If when re-starting the server you get an error binding to 0.0.0.0:8000 because the address is already in use, use this command to kill the process occupying the address and try again.  Make sure the address isn't already being used by a different application. 
   ```sh
   make kill
   ```

### Endpoints

* `/api/healthcheck` **GET** A simple health check endpoint with a helpful message.
* `/api/tables/{table_number}?page=1&limit=10` **GET** all the orders from table `{table_number}` with optional pagination.
* `/api/orders/{id}` **GET** a single order with `{id}`.
* `/api/tables/{table_number}` **POST** 1-10 orders to `{table_number}`.
* `/api/orders/{id}` **DELETE** a single order with `{id}`.

_NOTE: No UPDATE endpoint since no update functionality was requested._
