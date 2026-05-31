# Input Validation Guide

## Overview
A validação de input foi implementada usando a crate `validator` com as seguintes regras:

### Regras de Validação

**NewNoteInput (POST /notes)**
- `content`: 
  - Mínimo: 1 caractere
  - Máximo: 5000 caracteres
  - Mensagem de erro: "Content must be between 1 and 5000 characters"

**UpdateNoteInput (PATCH /notes/:id)**
- `content`: 
  - Mínimo: 1 caractere
  - Máximo: 5000 caracteres
  - Mensagem de erro: "Content must be between 1 and 5000 characters"

## Testing

### Unit Tests
Todos os testes de validação foram implementados em `src/routes/notes_tests.rs`:

```bash
cargo test --lib
```

Testes implementados:
- ✅ `test_valid_note_input` - Entrada válida passa
- ✅ `test_empty_note_input` - Content vazio é rejeitado
- ✅ `test_note_input_too_long` - Content > 5000 chars é rejeitado
- ✅ `test_note_input_at_max_length` - Content com exatamente 5000 chars passa
- ✅ `test_update_input_validation` - Update válido passa
- ✅ `test_update_input_empty` - Update vazio é rejeitado
- ✅ `test_update_input_too_long` - Update > 5000 chars é rejeitado

### Integration Tests with curl

#### 1. Criar nota com conteúdo válido ✅
```bash
curl -X POST http://localhost:3000/notes \
  -H "Content-Type: application/json" \
  -d '{"content": "Minha primeira nota"}'
```

**Resposta esperada:**
```json
{
  "id": 1,
  "content": "Minha primeira nota",
  "created_at": "2024-05-31T12:34:56.789Z"
}
```

#### 2. Criar nota com conteúdo vazio ❌
```bash
curl -X POST http://localhost:3000/notes \
  -H "Content-Type: application/json" \
  -d '{"content": ""}'
```

**Resposta esperada (400 Bad Request):**
```json
{
  "error": "Content must be between 1 and 5000 characters"
}
```

#### 3. Criar nota com conteúdo > 5000 caracteres ❌
```bash
curl -X POST http://localhost:3000/notes \
  -H "Content-Type: application/json" \
  -d "{\"content\": \"$(head -c 5001 /dev/zero | tr '\0' 'x')\"}"
```

**Resposta esperada (400 Bad Request):**
```json
{
  "error": "Content must be between 1 and 5000 characters"
}
```

#### 4. Atualizar nota com conteúdo válido ✅
```bash
curl -X PATCH http://localhost:3000/notes/1 \
  -H "Content-Type: application/json" \
  -d '{"content": "Conteúdo atualizado"}'
```

**Resposta esperada:**
```json
{
  "id": 1,
  "content": "Conteúdo atualizado",
  "created_at": "2024-05-31T12:34:56.789Z"
}
```

#### 5. Atualizar nota com conteúdo vazio ❌
```bash
curl -X PATCH http://localhost:3000/notes/1 \
  -H "Content-Type: application/json" \
  -d '{"content": ""}'
```

**Resposta esperada (400 Bad Request):**
```json
{
  "error": "Content must be between 1 and 5000 characters"
}
```

#### 6. Tentar atualizar nota inexistente
```bash
curl -X PATCH http://localhost:3000/notes/999 \
  -H "Content-Type: application/json" \
  -d '{"content": "Novo conteúdo"}'
```

**Resposta esperada (404 Not Found):**
```json
{
  "error": "resource not found"
}
```

## Como Adicionar Mais Validações

Se você quiser adicionar mais validações (ex: caracteres especiais, profanidade), edite `src/models.rs`:

```rust
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewNoteInput {
    #[validate(
        length(min = 1, max = 5000),
        custom = "validate_no_profanity"
    )]
    pub content: String,
}

fn validate_no_profanity(content: &str) -> Result<(), validator::ValidationError> {
    if content.contains("banned_word") {
        return Err(validator::ValidationError::new("profanity"));
    }
    Ok(())
}
```

## Próximos Passos

- Implementar testes de integração com banco de dados real
- Adicionar validações de rate limiting
- Implementar sanitização HTML se necessário
- Adicionar mais regras de negócio (ex: não permitir duplicatas)

## Recursos

- [Validator Crate Documentation](https://docs.rs/validator)
- [Axum Error Handling](https://docs.rs/axum/latest/axum/response/)
- [HTTP Status Codes](https://httpwg.org/specs/rfc9110.html#status.codes)
