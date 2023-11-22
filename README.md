<!-- ABOUT THE PROJECT -->
# Rust Restaurant API
## About The Project

A simple Rust api for retrieving, adding, and deleting orders from a restaurant table.<br />
Made using Axum with PostgreSQL in roughly 6 dev hours.  A pre-filled .env file is included for convenience.<br /><br />
_**Note:** When the backend starts 7 fake users will start making various requests to the server at random intervals._


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
3. Install dependencies
   ```sh
   make init
   ```
4. Start up the backend at: `localhost:8000`
   ```sh
   make start
   ```
5. Start up the frontend at: `localhost:3000`
   ```sh
   make front
   ```

### Additional Commands

* Shut down the docker database
   ```sh
   make stop
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

* **GET** A simple health check endpoint with a helpful message.<br />
   `/api/healthcheck`<br /><br />
   Example response: <br />
  ```json
  {
    "message": "Get a table's orders at api/tables/TABLE_NUMBER",
    "status": "success"
  }
  ```
  <br />

* **GET** all the orders from table `{table_number}`.<br />
   `/api/tables/{table_number}`<br /><br />
   Example response: <br />
   ```json
   {
    "orders": [
        {
            "cookTime": 15,
            "createdAt": "2023-11-22T09:50:45.589172Z",
            "id": "8fa51fdf-bdbc-4c98-8b77-574033a444b4",
            "item": "test food three",
            "tableNumber": 50
        },
        {
            "cookTime": 30,
            "createdAt": "2023-11-22T09:50:45.589172Z",
            "id": "4dd6f7a5-115f-4e89-b4b5-f4af6ed9f113",
            "item": "test food two",
            "tableNumber": 50
        },
        {
            "cookTime": 1,
            "createdAt": "2023-11-22T09:50:45.589172Z",
            "id": "20902598-667b-4a58-b8fd-d1e22fc2961a",
            "item": "test food one",
            "tableNumber": 50
        }
    ],
    "results": 3,
    "status": "success"
   }
   ```
   <br />
   
* **GET** a single order with `{id}`.<br />
   `/api/orders/{id}`<br /><br />
   Example response: <br />
   ```json
  {
    "order": {
        "cookTime": 1,
        "createdAt": "2023-11-22T09:50:45.589172Z",
        "id": "20902598-667b-4a58-b8fd-d1e22fc2961a",
        "item": "test food one",
        "tableNumber": 50
    },
    "status": "success"
  }
   ```
   <br />
   
* **POST** 1-10 orders to `{table_number}`.<br />
   `/api/tables/{table_number}`<br /><br />
   Example body: (note that `cook_time` is optional) <br />
   ```json
   {
    "orders": [{
            "item": "test food one",
            "cook_time": 1
        },
        {
            "item": "test food two",
            "cook_time": 30
        },
        {
            "item": "test food three"
        }
    ]
   }
   ```
   Example response:
   ```json
   {
    "orders": [
        {
            "cookTime": 1,
            "createdAt": "2023-11-22T09:50:45.589172Z",
            "id": "20902598-667b-4a58-b8fd-d1e22fc2961a",
            "item": "test food one",
            "tableNumber": 50
        },
        {
            "cookTime": 30,
            "createdAt": "2023-11-22T09:50:45.589172Z",
            "id": "4dd6f7a5-115f-4e89-b4b5-f4af6ed9f113",
            "item": "test food two",
            "tableNumber": 50
        },
        {
            "cookTime": 15,
            "createdAt": "2023-11-22T09:50:45.589172Z",
            "id": "8fa51fdf-bdbc-4c98-8b77-574033a444b4",
            "item": "test food three",
            "tableNumber": 50
        }
    ],
    "results": 3,
    "status": "success"
   }
   ```
   <br />
   
* **DELETE** a single order with `{id}`.<br />
   `/api/orders/{id}`<br />
   Returns status `204 No Content`

_NOTE: No UPDATE endpoint since no update functionality was requested._
