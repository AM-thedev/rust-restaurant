<!-- ABOUT THE PROJECT -->
# Rust Restaurant API
## About The Project

A simple Rust api for retrieving, adding, and deleting orders from a restaurant table.<br />
Made using Axum with PostgreSQL in roughly 6 dev hours.  A pre-filled .env file is included for convenience.<br /><br />
_**Note:** When the backend starts 5 fake users will start making various requests to the server at random intervals._


<!-- GETTING STARTED -->
## Getting Started

If you want to test the api manually, simply import the `Rust Restaurant Api.postman.json` file into Postman.<br />
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
   make front
   ```
* Run backend tests
   ```sh
   make test
   ```
* You may encounter an issue with bash where ctrl+C does not shut down the backend server, in that case consider closing the terminal entirely or running the following command to kill the process occupying the backend address (`localhost:8000`).
   ```sh
   make kill
   ```
* If you encounter the same issue with the frontend, consider closing the terminal entirely or running the following command to kill the process occupying the frontend address (`localhost:3000`).
   ```sh
   make front-kill
   ```

### Endpoints

* `/api/healthcheck`<br />
  **GET** A simple health check endpoint with a helpful message.<br /><br />
* `/api/tables/{table_number}`<br />
  **GET** all the orders from table `{table_number}`.<br /><br />
* `/api/orders/{id}`<br />
  **GET** a single order with `{id}`.<br /><br />
* `/api/tables/{table_number}`<br />
  **POST** 1-10 orders to `{table_number}`.<br /><br />
* `/api/orders/{id}`<br />
  **DELETE** a single order with `{id}`.

_NOTE: No UPDATE endpoint since no update functionality was requested._