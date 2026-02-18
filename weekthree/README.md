# Week Three - User Management REST API

A RESTful API built with Rust using Actix-web and PostgreSQL for managing users. This project demonstrates CRUD operations with async/await patterns and database integration.

## Overview

This is a simple user management REST API that allows you to create, read, update, and delete user records from a PostgreSQL database. The API is built using the Actix-web framework and uses SQLx for type-safe database queries.

## Features

- ✅ Create new users
- ✅ Retrieve all users
- ✅ Retrieve a single user by ID
- ✅ Update user information
- ✅ Delete users
- ✅ UUID-based user identification
- ✅ Type-safe database queries with SQLx
- ✅ Async/await architecture

## Project Structure

```
weekthree/
├── Cargo.toml           # Project dependencies and metadata
├── README.md            # This file
└── src/
    ├── main.rs          # Application entry point and server setup
    ├── db.rs            # Database connection initialization
    ├── models.rs        # Data models and structures
    ├── handlers.rs      # Request handlers for each endpoint
    └── routes.rs        # Route definitions
```

## Tech Stack

- **Language:** Rust
- **Web Framework:** Actix-web 4.x
- **Database:** PostgreSQL
- **ORM/Query Builder:** SQLx 0.7
- **Runtime:** Tokio
- **Serialization:** Serde
- **ID Generation:** UUID v4

## Prerequisites

- Rust 1.70 or later
- PostgreSQL 12 or later
- Cargo (comes with Rust)

## Installation

### 1. Clone the repository

```bash
cd weekthree
```

### 2. Install dependencies

```bash
cargo build
```

### 3. Set up the database

Create a PostgreSQL database and set up the users table:

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);
```

### 4. Configure environment variables

Create a `.env` file in the project root:

```
DATABASE_URL=postgres://username:password@localhost:5432/your_database_name
```

Replace the credentials with your PostgreSQL connection details.

## Running the Application

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`

## API Endpoints

### 1. Create a New User

**Endpoint:** `POST /users`

**Request Body:**
```json
{
  "name": "John Doe",
  "email": "john@example.com"
}
```

**Response:** `201 Created`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "John Doe",
  "email": "john@example.com"
}
```

---

### 2. Get All Users

**Endpoint:** `GET /users`

**Response:** `200 OK`
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john@example.com"
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "Jane Smith",
    "email": "jane@example.com"
  }
]
```

---

### 3. Get a Single User

**Endpoint:** `GET /users/{id}`

**Path Parameters:**
- `id` (UUID): The user's unique identifier

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "John Doe",
  "email": "john@example.com"
}
```

**Error Response:** `404 Not Found` if user doesn't exist

---

### 4. Update a User

**Endpoint:** `PUT /users/{id}`

**Path Parameters:**
- `id` (UUID): The user's unique identifier

**Request Body:** (both fields are optional)
```json
{
  "name": "John Updated",
  "email": "john.updated@example.com"
}
```

**Response:** `200 OK`

**Error Responses:**
- `404 Not Found` if user doesn't exist
- `500 Internal Server Error` on database error

---

### 5. Delete a User

**Endpoint:** `DELETE /users/{id}`

**Path Parameters:**
- `id` (UUID): The user's unique identifier

**Response:** `204 No Content`

**Error Responses:**
- `404 Not Found` if user doesn't exist
- `500 Internal Server Error` on database error

## Example Usage with cURL

### Create a user
```bash
curl -X POST http://127.0.0.1:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'
```

### Get all users
```bash
curl http://127.0.0.1:8080/users
```

### Get a specific user
```bash
curl http://127.0.0.1:8080/users/550e8400-e29b-41d4-a716-446655440000
```

### Update a user
```bash
curl -X PUT http://127.0.0.1:8080/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice Updated"}'
```

### Delete a user
```bash
curl -X DELETE http://127.0.0.1:8080/users/550e8400-e29b-41d4-a716-446655440000
```

## Data Models

### User
```rust
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
}
```

### CreateUser
```rust
pub struct CreateUser {
    pub name: String,
    pub email: String,
}
```

### UpdateUser
```rust
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}
```

## Dependencies

- **actix-web:** Web framework
- **sqlx:** Async SQL toolkit with macros for compile-time query checking
- **serde & serde_json:** JSON serialization/deserialization
- **tokio:** Async runtime
- **uuid:** UUID generation and parsing
- **dotenvy:** Environment variable management

## Key Features Explained

### Async/Await
All endpoints use async handlers with Tokio runtime for non-blocking I/O operations.

### Type-Safe Queries
SQLx provides compile-time query validation, ensuring database queries are correct before runtime.

### UUID Identifiers
Users are identified by UUIDs (Universally Unique Identifiers) instead of sequential IDs, improving security and distributed system compatibility.

### Connection Pooling
PostgreSQL connections are pooled using SQLx's built-in connection pool for better performance under load.

## Error Handling

The API returns appropriate HTTP status codes:

- **201 Created:** User successfully created
- **200 OK:** Request successful
- **204 No Content:** Resource deleted successfully
- **404 Not Found:** Resource not found
- **500 Internal Server Error:** Database or server error

## Troubleshooting

### Database Connection Error
Ensure:
- PostgreSQL is running
- `DATABASE_URL` environment variable is correctly set
- Database credentials are valid
- Network connectivity to the database server

### UUID Feature Error
Ensure `sqlx` dependency includes the `uuid` feature in `Cargo.toml`:
```toml
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio", "macros", "uuid"] }
```

## Development Tips

- Use `cargo check` for faster compilation checks
- Use `cargo test` to run tests (if added)
- Use `cargo clippy` for linting suggestions
- Use `RUST_LOG=debug cargo run` for debug logging

## Future Enhancements

- Add user authentication and authorization
- Add input validation
- Add pagination for GET /users
- Add filtering and search capabilities
- Add request/response logging
- Add unit and integration tests
- Add API documentation with OpenAPI/Swagger
- Add rate limiting
- Add database migrations

## License

This project is part of the MPL RUST BOOTCAMP internship program.

## Contact

For questions or issues, please refer to the bootcamp documentation or contact your instructor.
