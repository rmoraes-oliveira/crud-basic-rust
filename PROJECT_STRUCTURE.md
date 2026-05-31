# Project Structure

## File Organization

```
notes-api/
│
├── src/
│   ├── main.rs                 # Application entry point, server initialization
│   ├── models.rs              # Data models (Note, NewNoteInput)
│   ├── errors.rs              # Error handling and HTTP responses
│   │
│   ├── db/
│   │   ├── mod.rs             # Database module exports
│   │   └── notes.rs           # Database operations (list, get, create, delete)
│   │
│   └── routes/
│       ├── mod.rs             # Routes configuration and setup
│       └── notes.rs           # HTTP handlers (list, get, create, delete)
│
├── migrations/
│   └── 20260531215518_create_table_notes.sql  # Database schema
│
├── Dockerfile                  # Multi-stage Docker image
├── docker-compose.yml         # Docker Compose configuration
├── .dockerignore              # Files to exclude from Docker build
│
├── Cargo.toml                 # Project manifest and dependencies
├── Cargo.lock                 # Locked dependency versions
│
├── README.md                  # Main project documentation
├── DOCKER.md                  # Docker setup and usage guide
├── Makefile                   # Convenient command shortcuts
├── PROJECT_STRUCTURE.md       # This file
│
├── .env.example               # Environment variables template
├── .gitignore                 # Git ignore rules
│
└── target/                    # Compiled binaries and artifacts (generated)
```

## Module Hierarchy

```
notes-api
├── models         - Data structures
├── errors         - Error handling
├── db
│   └── notes      - Database layer
├── routes
│   └── notes      - HTTP handlers
└── main           - Application setup and server
```

## Key Files Description

### Core Application
- **main.rs**: Initializes Tokio runtime, loads environment, connects to database, sets up HTTP server
- **models.rs**: Defines `Note` and `NewNoteInput` structs with serialization
- **errors.rs**: Custom `AppError` enum and HTTP response conversion

### Database Layer
- **db/mod.rs**: Exports the notes submodule
- **db/notes.rs**: CRUD operations with SQLx queries
  - `list()` - Fetch all notes
  - `get_by_id()` - Fetch note by ID
  - `create()` - Insert new note
  - `delete()` - Remove note by ID

### HTTP Layer
- **routes/mod.rs**: Defines router and combines routes
- **routes/notes.rs**: Handler functions for HTTP endpoints
  - `list()` - GET /notes
  - `get()` - GET /notes/:id
  - `create()` - POST /notes
  - `delete()` - DELETE /notes/:id

### Configuration
- **Dockerfile**: Two-stage build for minimal image size
- **docker-compose.yml**: PostgreSQL and API service definitions
- **Cargo.toml**: Dependencies and project metadata

### Documentation
- **README.md**: Getting started, API documentation, examples
- **DOCKER.md**: Docker-specific instructions and troubleshooting
- **Makefile**: Common development and deployment tasks

## Data Flow

```
HTTP Request
    ↓
routes/notes.rs (Handler)
    ↓
db/notes.rs (Database Query)
    ↓
PostgreSQL Database
    ↓
db/notes.rs (Result)
    ↓
models.rs (Serialization)
    ↓
HTTP Response (JSON)
```

## Dependencies

### Web Framework
- **axum** - Modern web framework for Rust
- **tokio** - Async runtime

### Database
- **sqlx** - Compile-time checked SQL
- **postgres** - PostgreSQL driver

### Serialization
- **serde** - Serialization/deserialization
- **serde_json** - JSON support

### Utilities
- **chrono** - Date and time
- **dotenvy** - Environment variables

## Type Definitions

### Note (Database Model)
```rust
pub struct Note {
    pub id: i32,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}
```

### NewNoteInput (Request Body)
```rust
pub struct NewNoteInput {
    pub content: String,
}
```

### AppError (Error Handling)
```rust
pub enum AppError {
    Database(sqlx::Error),
    NotFound,
}
```

### AppState (Shared State)
```rust
pub struct AppState {
    pub db: PgPool,
}
```

## Naming Conventions

- **Files**: snake_case (e.g., `notas.rs` → `notes.rs`)
- **Modules**: snake_case (e.g., `mod notes`)
- **Functions**: snake_case (e.g., `list_notes` → `list`)
- **Structs**: PascalCase (e.g., `Note`, `AppError`)
- **Constants**: UPPER_SNAKE_CASE

## API Endpoints

| Method | Path | Handler | Operation |
|--------|------|---------|-----------|
| GET | /notes | list | List all notes |
| GET | /notes/:id | get | Get note by ID |
| POST | /notes | create | Create new note |
| DELETE | /notes/:id | delete | Delete note |

## Database Schema

```sql
CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## Environment Variables

- **DATABASE_URL**: PostgreSQL connection string
  - Default: `postgresql://postgres:postgres@localhost:5432/notes_db`
- **RUST_LOG**: Logging level (optional)
  - Default: `info`

## Build Process

### Local Development
```
Cargo.toml → cargo build → target/debug/notes-api
```

### Docker Production
```
Dockerfile (Builder Stage) → cargo build --release → Dockerfile (Runtime Stage) → Docker Image
```

## Port Usage

- **3000**: HTTP API server
- **5432**: PostgreSQL database (Docker)

## Migration Strategy

- Migrations stored in `migrations/` directory
- SQLx manages migration state in `_sqlx_migrations` table
- One migration file per schema change
- Reversible migrations supported with `-r` flag
