# Notes API

A simple REST API for managing notes built with Rust, Axum, and PostgreSQL.

## Features

- ✅ Create, read, update, and delete notes
- ✅ Input validation (1-5000 character limit)
- ✅ Pagination with limit/offset
- ✅ JWT authentication with Bearer tokens
- ✅ Structured logging with tracing
- ✅ Built with Axum web framework
- ✅ PostgreSQL database with SQLx
- ✅ Async/await with Tokio
- ✅ Docker support
- ✅ Database migrations with SQLx
- ✅ 17 unit tests + integration tests

## Prerequisites

- Rust 1.70+ ([Install](https://rustup.rs/))
- PostgreSQL 12+ (or use Docker)
- Docker & Docker Compose (optional)

## Installation

### Local Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd notes-api
```

2. Create a `.env` file:
```bash
cp .env.example .env
```

3. Configure your database URL in `.env`:
```
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/notes_db
```

4. Create the database and run migrations:
```bash
sqlx database create
sqlx migrate run
```

5. Run the development server:
```bash
cargo run
```

The API will be available at `http://localhost:3000`

### Docker Setup

1. Start services with Docker Compose:
```bash
docker-compose up -d
```

2. Run migrations inside the container:
```bash
docker-compose exec api sqlx migrate run
```

3. Access the API at `http://localhost:3000`

Stop the services:
```bash
docker-compose down
```

## Quick Start

### 1. Login and Get Token
```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username": "user123", "password": "password123"}'
```

Response:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### 2. Use Token to Access Notes
```bash
curl -H "Authorization: Bearer <token>" \
  http://localhost:3000/notes
```

---

## API Endpoints

### List all notes (with pagination)
```http
GET /notes?limit=20&offset=0
```

Query Parameters:
- `limit` (optional, default: 20) - Items per page
- `offset` (optional, default: 0) - Items to skip

### Get a specific note
```http
GET /notes/:id
```

### Create a new note
```http
POST /notes
Content-Type: application/json

{
  "content": "My first note"
}
```

### Update a note
```http
PATCH /notes/:id
Content-Type: application/json

{
  "content": "Updated note content"
}
```

### Delete a note
```http
DELETE /notes/:id
```

## Example Requests

### Create a note
```bash
curl -X POST http://localhost:3000/notes \
  -H "Content-Type: application/json" \
  -d '{"content": "Learn Rust"}'
```

### List all notes (default pagination)
```bash
curl http://localhost:3000/notes
```

### List notes with custom pagination
```bash
curl "http://localhost:3000/notes?limit=10&offset=20"
```

Response includes pagination metadata:
```json
{
  "data": [...],
  "pagination": {
    "limit": 10,
    "offset": 20,
    "total": 100
  }
}
```

### Get a specific note
```bash
curl http://localhost:3000/notes/1
```

### Update a note
```bash
curl -X PATCH http://localhost:3000/notes/1 \
  -H "Content-Type: application/json" \
  -d '{"content": "Updated content"}'
```

### Delete a note
```bash
curl -X DELETE http://localhost:3000/notes/1
```

## Validation

All endpoints validate input with the following rules:
- **content**: Must be between 1 and 5000 characters
- Invalid requests return `400 Bad Request` with error details

See [VALIDATION_GUIDE.md](VALIDATION_GUIDE.md) for detailed validation examples.

## Testing

### Unit Tests
```bash
cargo test --lib
```

### Integration Tests (with PostgreSQL)
```bash
cargo test --test integration_tests
```

### All Tests
```bash
cargo test
```

**Test Summary:** 12 tests total (11 unit + 1 integration), 100% passing

## Documentation

- [TESTING_GUIDE.md](TESTING_GUIDE.md) - Unit and integration tests
- [VALIDATION_GUIDE.md](VALIDATION_GUIDE.md) - Input validation rules
- [PAGINATION_GUIDE.md](PAGINATION_GUIDE.md) - Pagination with limit/offset
- [AUTHENTICATION_GUIDE.md](AUTHENTICATION_GUIDE.md) - JWT auth and login
- [LOGGING_GUIDE.md](LOGGING_GUIDE.md) - Structured logging configuration
- [LEARNING_PARETO.md](LEARNING_PARETO.md) - Learning roadmap and next steps

## Project Structure

```
notes-api/
├── src/
│   ├── main.rs           # Application entry point
│   ├── models.rs         # Data models (Note, NewNoteInput)
│   ├── errors.rs         # Error handling
│   ├── db/
│   │   ├── mod.rs        # Database module
│   │   └── notes.rs      # Note database operations
│   └── routes/
│       ├── mod.rs        # Routes configuration
│       └── notes.rs      # Note handlers
├── migrations/           # Database migrations
├── Cargo.toml           # Project dependencies
├── Dockerfile           # Docker image configuration
├── docker-compose.yml   # Multi-container setup
└── README.md            # This file
```

## Development

### Build the project
```bash
cargo build
```

### Run tests
```bash
cargo test
```

### Check code
```bash
cargo check
```

### Format code
```bash
cargo fmt
```

### Lint code
```bash
cargo clippy
```

## Database Migrations

Create a new migration:
```bash
sqlx migrate add -r <migration_name>
```

Run migrations:
```bash
sqlx migrate run
```

Revert last migration:
```bash
sqlx migrate revert
```

## Environment Variables

Create a `.env` file with the following variables:

```
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/notes_db
RUST_LOG=info
```

## Logging

View detailed logs during development:

```bash
# Run with info-level logs
RUST_LOG=info cargo run

# Run with debug-level logs
RUST_LOG=debug cargo run

# Filter specific modules
RUST_LOG=notes_api=debug,axum=info cargo run
```

See [LOGGING_GUIDE.md](LOGGING_GUIDE.md) for more details.

## Technology Stack

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL
- **Database Access**: SQLx
- **Async Runtime**: Tokio
- **Serialization**: Serde
- **DateTime**: Chrono

## Dependencies

Key dependencies in `Cargo.toml`:
- `axum` - Web framework
- `sqlx` - SQL toolkit with compile-time checked queries
- `tokio` - Async runtime
- `serde` - Serialization framework
- `chrono` - Date and time handling
- `dotenvy` - Environment variable loading

## Error Handling

The API returns appropriate HTTP status codes:
- `200 OK` - Successful request
- `201 Created` - Resource successfully created
- `204 No Content` - Resource successfully deleted
- `400 Bad Request` - Invalid request
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

Error responses have this format:
```json
{
  "error": "resource not found"
}
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues and questions, please open an issue on the GitHub repository.
