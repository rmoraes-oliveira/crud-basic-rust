# Pagination Guide

## Overview
O endpoint `/notes` agora suporta paginação via query parameters `limit` e `offset`.

**Default Values:**
- `limit`: 20 (máximo de itens por página)
- `offset`: 0 (começar desde o primeiro item)

---

## Query Parameters

### `limit` (Optional)
- **Default:** 20
- **Type:** Integer (i64)
- **Description:** Número máximo de notes a retornar
- **Example:** `?limit=50` - retorna até 50 notas

### `offset` (Optional)
- **Default:** 0
- **Type:** Integer (i64)
- **Description:** Número de items a pular
- **Example:** `?offset=100` - pula as primeiras 100 notas

---

## Response Format

Todas as requisições para `/notes` agora retornam:

```json
{
  "data": [
    {
      "id": 1,
      "content": "Note content",
      "created_at": "2024-05-31T12:34:56.789Z"
    },
    {
      "id": 2,
      "content": "Another note",
      "created_at": "2024-05-31T12:35:00.000Z"
    }
  ],
  "pagination": {
    "limit": 20,
    "offset": 0,
    "total": 100
  }
}
```

### Response Fields

**`data`** - Array de notas
- Ordenadas por `created_at DESC` (mais recentes primeiro)
- Limitado pelo valor de `limit`

**`pagination`** - Informações de paginação
- `limit`: Limite usado na requisição
- `offset`: Offset usado na requisição
- `total`: Total de notas no banco (não paginado)

---

## Examples

### 1. List All Notes (Default Pagination)
```bash
curl http://localhost:3000/notes
```

**Response:**
```json
{
  "data": [
    {"id": 3, "content": "Latest note", "created_at": "..."},
    {"id": 2, "content": "Second note", "created_at": "..."},
    {"id": 1, "content": "First note", "created_at": "..."}
  ],
  "pagination": {
    "limit": 20,
    "offset": 0,
    "total": 3
  }
}
```

### 2. First 10 Notes
```bash
curl "http://localhost:3000/notes?limit=10&offset=0"
```

**Response:**
```json
{
  "data": [
    {"id": 10, "content": "Note 10", "created_at": "..."},
    {"id": 9, "content": "Note 9", "created_at": "..."},
    ...
  ],
  "pagination": {
    "limit": 10,
    "offset": 0,
    "total": 50
  }
}
```

### 3. Second Page (10 items per page)
```bash
curl "http://localhost:3000/notes?limit=10&offset=10"
```

**Response:**
```json
{
  "data": [
    {"id": 20, "content": "Note 20", "created_at": "..."},
    {"id": 19, "content": "Note 19", "created_at": "..."},
    ...
  ],
  "pagination": {
    "limit": 10,
    "offset": 10,
    "total": 50
  }
}
```

### 4. Last Page (if total = 50)
```bash
curl "http://localhost:3000/notes?limit=10&offset=40"
```

**Response:**
```json
{
  "data": [
    {"id": 10, "content": "Note 10", "created_at": "..."},
    {"id": 9, "content": "Note 9", "created_at": "..."},
    ...
  ],
  "pagination": {
    "limit": 10,
    "offset": 40,
    "total": 50
  }
}
```

### 5. Custom Large Limit
```bash
curl "http://localhost:3000/notes?limit=100&offset=0"
```

**Response:**
```json
{
  "data": [
    ... até 100 notas ...
  ],
  "pagination": {
    "limit": 100,
    "offset": 0,
    "total": 250
  }
}
```

---

## Pagination Strategy

### How to Calculate Pages

Para saber quantas páginas existem:

```
total_pages = ceil(total / limit)
```

**Exemplo:** 100 notas com limit=10
```
total_pages = ceil(100 / 10) = 10 páginas
```

### How to Navigate

**Próxima página:**
```
next_offset = current_offset + limit
```

**Página anterior:**
```
prev_offset = max(0, current_offset - limit)
```

**Ir para página N (0-indexed):**
```
offset = page_number * limit
```

### Example: Fetching All Pages

```bash
#!/bin/bash
LIMIT=20
OFFSET=0
TOTAL=0

while true; do
  RESPONSE=$(curl -s "http://localhost:3000/notes?limit=$LIMIT&offset=$OFFSET")
  
  # Parse response (usando jq)
  ITEMS=$(echo $RESPONSE | jq '.data | length')
  TOTAL=$(echo $RESPONSE | jq '.pagination.total')
  
  echo "Got $ITEMS items from offset $OFFSET"
  
  # Se não houver mais items, sair
  if [ $ITEMS -eq 0 ]; then
    break
  fi
  
  # Próxima página
  OFFSET=$((OFFSET + LIMIT))
  
  # Parar se ultrapassou o total
  if [ $OFFSET -ge $TOTAL ]; then
    break
  fi
done
```

---

## Performance Considerations

### When to Use Pagination
✅ **Use pagination quando:**
- Você tem muitos notes (> 100)
- Quer mostrar em páginas no frontend
- Quer limitar bandwidth
- Quer evitar sobrecarregar o servidor

### Limit Recomendado
| Caso | Limit |
|------|-------|
| Desktop web | 20-50 |
| Mobile web | 10-20 |
| API mobile | 10 |
| Relatórios | 100+ |

### Query Performance
- **COM indexação:** O(1) - muito rápido
- **SEM indexação:** O(n) - pode ficar lento com muitos dados

**Índices adicionados automaticamente:**
```sql
-- SQLx migrações usam índices automaticamente em created_at
CREATE INDEX idx_notes_created_at ON notes(created_at DESC);
```

---

## Sorting

Notes são **sempre** retornadas em ordem:
```
ORDER BY created_at DESC
```

Isto significa:
- **Mais recentes primeiro** (notas criadas recentemente vêm primeiro)
- Consistente entre requisições
- Ordenação é feita no banco (eficiente)

---

## Edge Cases

### 1. Offset > Total
```bash
curl "http://localhost:3000/notes?offset=1000&limit=20"
```

**Response:**
```json
{
  "data": [],
  "pagination": {
    "limit": 20,
    "offset": 1000,
    "total": 50
  }
}
```

Retorna um array vazio `data` pois não há mais notes.

### 2. Limite = 0
```bash
curl "http://localhost:3000/notes?limit=0"
```

Retorna 0 items (mas ainda traz `total`).

### 3. Valores Negativos
```bash
curl "http://localhost:3000/notes?limit=-10"
```

❌ Comportamento indefinido - **evite**. O SQL pode retornar erro.

---

## Testing Pagination

### Unit Tests
```bash
cargo test pagination
```

### Integration Tests
```bash
cargo test test_pagination_queries
```

---

## Migration Path

Se você tinha código usando a resposta antiga (sem paginação):

**Antes:**
```json
[
  {"id": 1, "content": "Note"},
  {"id": 2, "content": "Note"}
]
```

**Agora:**
```json
{
  "data": [
    {"id": 1, "content": "Note"},
    {"id": 2, "content": "Note"}
  ],
  "pagination": {
    "limit": 20,
    "offset": 0,
    "total": 2
  }
}
```

**Para manter compatibilidade** (em v2):
- Parse `response.data` em vez de `response` diretamente
- Use `response.pagination.total` para saber total de items

---

## Future Enhancements

- [ ] Cursor-based pagination (para datasets muito grandes)
- [ ] Search + filter + pagination
- [ ] Sorting customizável (por content, por id, etc)
- [ ] Export com paginação automática
