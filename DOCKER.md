# Docker Setup Guide

## Quick Start

### Prerequisites
- Docker 20.10+
- Docker Compose 2.0+

### Start the Application

1. Build and start all services:
```bash
docker compose up -d
```

2. Run migrations (first time only):
```bash
docker compose exec api sqlx migrate run
```

3. Check if services are running:
```bash
docker compose ps
```

Expected output:
```
CONTAINER ID   IMAGE                      COMMAND                  PORTS                    NAMES
xxxxxx         notes-api                  "./notes-api"            0.0.0.0:3000->3000/tcp  notes_api
xxxxxx         postgres:16-alpine         "postgres"               0.0.0.0:5432->5432/tcp  notes_postgres
```

### Test the API

Create a note:
```bash
curl -X POST http://localhost:3000/notes \
  -H "Content-Type: application/json" \
  -d '{"content": "Hello from Docker"}'
```

List all notes:
```bash
curl http://localhost:3000/notes
```

## Services

### PostgreSQL Database
- **Image**: postgres:16-alpine
- **Container Name**: notes_postgres
- **Port**: 5432
- **User**: postgres
- **Password**: postgres
- **Database**: notes_db
- **Volume**: postgres_data (persists data)

### API Server
- **Image**: Built from Dockerfile
- **Container Name**: notes_api
- **Port**: 3000
- **Health Check**: Database connectivity

## Common Commands

### View logs
```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f api
docker compose logs -f postgres
```

### Access PostgreSQL
```bash
docker compose exec postgres psql -U postgres -d notes_db
```

### Execute commands in container
```bash
# Run shell in API container
docker compose exec api bash

# Run migrations
docker compose exec api sqlx migrate run

# Run tests
docker compose exec api cargo test
```

### Stop services
```bash
# Stop but keep data
docker compose stop

# Stop and remove containers
docker compose down

# Remove volumes (delete data)
docker compose down -v
```

### Rebuild image
```bash
# Without cache
docker compose build --no-cache

# Rebuild and start
docker compose up -d --build
```

## Environment Configuration

Variables from `docker-compose.yml`:
```yaml
DATABASE_URL: postgresql://postgres:postgres@postgres:5432/notes_db
```

To change defaults, edit `docker-compose.yml` before starting services.

## Volume Management

### Persist database data
The `postgres_data` volume automatically persists your database. To manage:

```bash
# List volumes
docker volume ls

# Inspect volume
docker volume inspect notes-api_postgres_data

# Remove volume (WARNING: deletes data)
docker volume rm notes-api_postgres_data
```

## Troubleshooting

### Container fails to start
```bash
# Check logs
docker compose logs api

# Ensure ports 3000 and 5432 are not in use
lsof -i :3000
lsof -i :5432
```

### Database connection errors
```bash
# Verify PostgreSQL is running
docker compose exec postgres pg_isready

# Check DATABASE_URL
docker compose exec api echo $DATABASE_URL
```

### Migrations not applied
```bash
# Run manually
docker compose exec api sqlx migrate run

# Check migration status
docker compose exec api sqlx migrate info
```

### Out of disk space
```bash
# Clean up Docker resources
docker system prune -a

# Remove unused volumes
docker volume prune
```

## Production Deployment

For production, consider:

1. **Use Docker secrets** instead of environment variables
2. **Enable PostgreSQL backups**:
   ```bash
   docker compose exec postgres pg_dump -U postgres notes_db > backup.sql
   ```

3. **Configure resource limits** in docker-compose.yml:
   ```yaml
   services:
     api:
       deploy:
         resources:
           limits:
             cpus: '0.5'
             memory: 512M
   ```

4. **Use a production-grade reverse proxy** (nginx, Traefik)

5. **Enable SSL/TLS** for database connections

6. **Setup monitoring** (Prometheus, Grafana)

## Docker Image Details

### Build Stages
- **Builder**: Rust official image (compiles application)
- **Runtime**: Debian slim (small final image)

### Size Optimization
- Multi-stage build reduces final image size
- Only runtime dependencies included in final image
- Alpine PostgreSQL for smaller image

### Security
- Non-root user can be added in production
- Minimal dependencies in runtime image
- CA certificates included for HTTPS

## References

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Reference](https://docs.docker.com/compose/compose-file/)
- [PostgreSQL Docker Image](https://hub.docker.com/_/postgres)
- [Rust Docker Official Image](https://hub.docker.com/_/rust)
