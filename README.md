# CRUD in RUST - Groceries Management

This project is a REST API developed in Rust for managing a list of groceries. The API provides endpoints for performing CRUD (Create, Read, Update, Delete) operations on grocery items (to begin with, we'll just use name and value).

## Features

- **GET /v1/groceries**: Retrieve the list of all groceries.
- **POST /v1/groceries**: Add a new grocery item.
- **PUT /v1/groceries**: Update an existing grocery item by name.
- **DELETE /v1/groceries**: Delete a grocery item by name.

## Setup and Installation

### Prerequisites

- Rust 1.83.0
- tokio 0.2
- warp 0.2
- serde 1.0
- parking_lot 0.10.0

### Installation Steps

1. Clone the repository:
   ```sh
   git clone https://github.com/ronymarcolino/crud-in-rust.git
   cd crud-in-rust
   ```

2. Build the project:
   ```sh
   cargo build
   ```

3. Run the project:
   ```sh
   cargo run
   ```

The API will be accessible at `http://localhost:3030`.

## Usage

### Endpoints

#### GET /v1/groceries
Retrieve the list of all groceries.

**Response:**
- `200 OK`: Returns an array of grocery items.

#### POST /v1/groceries
Add a new grocery item.

**Request Body:**
- JSON object containing `name` (string) and `quantity` (integer).

**Response:**
- `201 Created`: Returns the created grocery item.

#### PUT /v1/groceries/
Update an existing grocery item by name.

**Request Body:**
- JSON object containing `name` (string) and/or `quantity` (integer).

**Response:**
- `200 OK`: Returns the updated grocery item.
- `404 Not Found`: If the grocery item does not exist.

#### DELETE /v1/groceries
Delete a grocery item by name.

**Response:**
- `200 OK`: If the grocery item was successfully deleted.
- `404 Not Found`: If the grocery item does not exist.

## License

This project is licensed under the MIT License.

## References

https://blog.logrocket.com/building-rest-api-rust-warp/
