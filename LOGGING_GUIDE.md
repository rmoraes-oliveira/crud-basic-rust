# Logging Guide

## Overview
A API usa **`tracing`** para logging estruturado. Logs incluem contexto estruturado, níveis de severidade e timestamps automáticos.

---

## Log Levels

| Level | Severity | Usage |
|-------|----------|-------|
| **TRACE** | Lowest | Detalhes muito granulares (raramente usado) |
| **DEBUG** | Low | Informações de debug para desenvolvimento |
| **INFO** | Medium | Operações importantes, request/response |
| **WARN** | High | Comportamentos inesperados, não é erro |
| **ERROR** | Highest | Erros, falhas, situações anormais |

---

## Configuring Log Level

### Via Environment Variable
```bash
# Only show INFO and above
RUST_LOG=info cargo run

# Only show DEBUG and above
RUST_LOG=debug cargo run

# Only show TRACE (most verbose)
RUST_LOG=trace cargo run

# No logs (ERROR only)
RUST_LOG=error cargo run
```

### Per-Module Control
```bash
# notes-api at debug, everything else at info
RUST_LOG=notes_api=debug,info cargo run

# Only axum debug logs, rest info
RUST_LOG=axum=debug,info cargo run

# Multiple modules
RUST_LOG=notes_api=debug,sqlx=trace,info cargo run
```

### Default
If not set:
```bash
RUST_LOG=info  # Default in the code
```

---

## Current Logging Implementation

### Server Startup
```
[INFO] Starting Notes API server
[INFO] Connecting to database
[INFO] Running migrations
[INFO] server running at http://localhost:3000
```

### List Notes
```
[INFO] listing notes, limit=20, offset=0
[INFO] notes retrieved successfully, count=5, total=100
```

### Get Note
```
[INFO] fetching note, note_id=1
[INFO] note retrieved successfully, note_id=1
[ERROR] note not found, note_id=999
```

### Create Note
```
[INFO] creating note, content_length=25
[INFO] note created successfully, note_id=1
[ERROR] validation error: Content must be between 1 and 5000 characters
```

### Update Note
```
[INFO] updating note, note_id=1, content_length=30
[INFO] note updated successfully, note_id=1
[ERROR] note not found for update, note_id=999
```

### Delete Note
```
[INFO] deleting note, note_id=1
[INFO] note deleted successfully, note_id=1
[ERROR] note not found for deletion, note_id=999
```

### Authentication
```
[INFO] login attempt, username=user123
[INFO] login successful, username=user123
[ERROR] validation error: ...
[WARN] Missing Authorization header
[WARN] Invalid Authorization header format
[WARN] Failed to decode JWT: ...
```

---

## Reading Logs

### Run with INFO level
```bash
RUST_LOG=info cargo run
```

Output:
```
2024-05-31T12:34:56.789Z INFO  notes_api: Starting Notes API server
2024-05-31T12:34:57.001Z INFO  notes_api: Connecting to database
2024-05-31T12:34:57.234Z INFO  notes_api: Running migrations
2024-05-31T12:34:57.456Z INFO  notes_api: server running at http://localhost:3000
```

### Run with DEBUG level
```bash
RUST_LOG=debug cargo run
```

Shows more detailed information:
```
2024-05-31T12:34:56.789Z DEBUG axum: [...]
2024-05-31T12:34:57.001Z DEBUG sqlx: query...
```

---

## Log Examples

### Complete Request Flow
```bash
RUST_LOG=info cargo run
```

Request:
```bash
curl -X POST http://localhost:3000/notes \
  -H "Content-Type: application/json" \
  -d '{"content": "My note"}'
```

Logs:
```
[INFO] creating note, content_length=7
[INFO] note created successfully, note_id=1
```

### With Validation Error
```bash
curl -X POST http://localhost:3000/notes \
  -H "Content-Type: application/json" \
  -d '{"content": ""}'
```

Logs:
```
[INFO] creating note, content_length=0
[ERROR] validation error: Content must be between 1 and 5000 characters
```

### List with Pagination
```bash
curl "http://localhost:3000/notes?limit=10&offset=0"
```

Logs:
```
[INFO] listing notes, limit=10, offset=0
[INFO] notes retrieved successfully, count=10, total=100
```

---

## Structured Logging

Logs are structured with key-value pairs for easy parsing:

```
[INFO] creating note, content_length=25
       └─ structured field
```

**Parsing in ELK / Log Aggregation:**
```json
{
  "level": "INFO",
  "message": "creating note",
  "content_length": 25,
  "timestamp": "2024-05-31T12:34:56.789Z"
}
```

---

## Using Logs in Development

### Finding Issues
```bash
# All errors
RUST_LOG=info cargo run 2>&1 | grep ERROR

# All notes operations
RUST_LOG=info cargo run 2>&1 | grep "note"

# Specific note
RUST_LOG=info cargo run 2>&1 | grep "note_id=1"
```

### Performance Investigation
```bash
RUST_LOG=debug cargo run 2>&1 | grep "sqlx"
# Shows all database queries
```

### Authentication Issues
```bash
RUST_LOG=info cargo run 2>&1 | grep -i "auth\|token"
# Shows all auth-related logs
```

---

## Adding Logs to Your Code

### In Handlers
```rust
use tracing::info;

pub async fn my_handler() {
    info!("starting operation");
    // do something
    info!(result = "success", "operation complete");
}
```

### With Context
```rust
info!(note_id = 123, user_id = 5, "processing note");

info!(
    status = "created",
    content_length = 250,
    "note operation complete"
);
```

### Error Logging
```rust
use tracing::error;

error!(note_id = 123, "note not found");
error!("database error: {}", e);
```

### Warning
```rust
use tracing::warn;

warn!("slow operation took {}ms", duration);
```

---

## Log Aggregation (Production)

### With ELK Stack (Elasticsearch, Logstash, Kibana)
1. Configure tracing to JSON format
2. Send logs to Logstash
3. Parse and index in Elasticsearch
4. Visualize in Kibana

### With Datadog
1. Add datadog tracing subscriber
2. Logs auto-collected by Datadog agent
3. Filter and alert in Datadog dashboard

### With Cloudwatch (AWS)
1. Configure CloudWatch appender
2. Logs sent to CloudWatch Logs
3. Create metrics and alarms

---

## Filtering Logs by Module

### Only Database Logs
```bash
RUST_LOG=sqlx=debug cargo run
```

### Only Axum/HTTP Logs
```bash
RUST_LOG=axum=debug cargo run
```

### Only Application Logs
```bash
RUST_LOG=notes_api=debug cargo run
```

### Combine Multiple
```bash
RUST_LOG=notes_api=info,axum=debug cargo run
```

---

## Best Practices

✅ **DO:**
- Log important operations (CREATE, UPDATE, DELETE)
- Log errors with context
- Use structured logging (key=value pairs)
- Include identifiers (note_id, user_id, request_id)

❌ **DON'T:**
- Log passwords or sensitive data
- Log sensitive user information
- Too verbose (spam logs)
- Inconsistent log messages

---

## Common Log Patterns

### Request/Response Cycle
```rust
info!("processing request");
let result = process();
info!("request complete");
```

### Error Handling
```rust
.map_err(|e| {
    error!("operation failed: {}", e);
    AppError::Internal
})?;
```

### Conditional Logging
```rust
if result.is_empty() {
    warn!("no results found");
}
```

---

## Troubleshooting

### No Logs Showing
```bash
# Check if RUST_LOG is set
echo $RUST_LOG

# Set to info
export RUST_LOG=info
cargo run
```

### Too Many Logs
```bash
# Filter specific module
RUST_LOG=notes_api=info cargo run

# Or increase log level
RUST_LOG=warn cargo run
```

### Logs Going to stderr
- This is normal
- Tracing writes to stderr by default
- Redirect: `cargo run 2>&1`

---

## Resources

- [tracing Documentation](https://docs.rs/tracing)
- [tracing-subscriber](https://docs.rs/tracing-subscriber)
- [Structured Logging Best Practices](https://www.kartar.net/2015/12/structured-logging/)
