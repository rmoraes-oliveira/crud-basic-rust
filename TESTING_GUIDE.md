# Testing Guide

## Overview
Este projeto possui três camadas de testes:

1. **Unit Tests** - Validação de regras de negócio
2. **Database Tests** - Testes com banco de dados real
3. **Integration Tests** - Testes completos CRUD com PostgreSQL

---

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Unit Tests Only
```bash
cargo test --lib
```

### Run Integration Tests Only
```bash
cargo test --test integration_tests
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Single Test
```bash
cargo test test_create_note
```

---

## Test Structure

### 1. Unit Tests (11 tests)

**Location:** `src/routes/notes_tests.rs` + `src/db/notes.rs`

Testes unitários focam em validação de input e operações simples.

#### Validation Tests (7 tests)
```rust
#[test]
fn test_valid_note_input() { ... }
#[test]
fn test_empty_note_input() { ... }
#[test]
fn test_note_input_too_long() { ... }
#[test]
fn test_note_input_at_max_length() { ... }
```

**Rodando:**
```bash
cargo test routes::notes_tests
```

#### Database Operation Tests (4 tests)
```rust
#[tokio::test]
async fn test_create_note() { ... }
#[tokio::test]
async fn test_get_nonexistent_note() { ... }
#[tokio::test]
async fn test_update_nonexistent_note() { ... }
#[tokio::test]
async fn test_delete_nonexistent_note() { ... }
```

**Rodando:**
```bash
cargo test db::notes::tests
```

### 2. Integration Tests (1 test)

**Location:** `tests/integration_tests.rs`

Teste de integração completo que valida o ciclo CRUD com PostgreSQL real.

```rust
#[tokio::test]
async fn test_database_integration() {
    // CREATE
    // READ
    // UPDATE
    // DELETE
    // VERIFY
}
```

**Rodando:**
```bash
cargo test --test integration_tests
```

---

## Test Results

### Current Coverage
- ✅ **12 tests total** (11 unit + 1 integration)
- ✅ **100% passing**
- ✅ Validation rules
- ✅ CRUD operations
- ✅ Error handling

### Summary
```
running 11 tests (unit tests)
test result: ok. 11 passed; 0 failed

running 1 test (integration)
test result: ok. 1 passed; 0 failed
```

---

## What Each Test Validates

### Validation Tests
| Test | What | Expected |
|------|------|----------|
| `test_valid_note_input` | Content válido (1-5000 chars) | ✅ Pass |
| `test_empty_note_input` | Content vazio | ❌ Validation error |
| `test_note_input_too_long` | Content > 5000 chars | ❌ Validation error |
| `test_note_input_at_max_length` | Content com exatamente 5000 chars | ✅ Pass |
| `test_update_input_validation` | Update válido | ✅ Pass |
| `test_update_input_empty` | Update vazio | ❌ Validation error |
| `test_update_input_too_long` | Update > 5000 chars | ❌ Validation error |

### Database Tests
| Test | Operation | Expected |
|------|-----------|----------|
| `test_create_note` | Criar nota | Retorna nota com ID |
| `test_get_nonexistent_note` | Buscar nota inexistente | Retorna None |
| `test_update_nonexistent_note` | Atualizar nota inexistente | Retorna None |
| `test_delete_nonexistent_note` | Deletar nota inexistente | Retorna false |

### Integration Test
| Operation | What | Expected |
|-----------|------|----------|
| INSERT | Criar nota com conteúdo | Insere com sucesso |
| SELECT | Buscar nota criada | Encontra nota |
| UPDATE | Atualizar conteúdo | Atualiza com sucesso |
| SELECT (after update) | Verificar atualização | Vê novo conteúdo |
| DELETE | Deletar nota | Deleta com sucesso |
| SELECT (after delete) | Verificar deleção | Não encontra nota |

---

## Running Tests with Docker

Se estiver usando Docker Compose para o PostgreSQL:

```bash
# Start services
docker-compose up -d

# Run tests
cargo test

# Stop services
docker-compose down
```

---

## Environment Variables

### Setup para testes

```bash
# Use a database separada para testes (opcional)
export TEST_DATABASE_URL=postgresql://user:password@localhost:5432/notes_test

# Ou use a mesma database (com cuidado!)
export DATABASE_URL=postgresql://user:password@localhost:5432/notes_db
```

---

## Adding New Tests

### Template para novo test unitário

```rust
#[test]
fn test_my_new_feature() {
    let input = MyInput { /* ... */ };
    let result = my_function(input);
    assert_eq!(result, expected);
}
```

### Template para novo test async

```rust
#[tokio::test]
async fn test_my_async_function() {
    let pool = setup_test_db().await;
    
    // Seu teste aqui
    
    cleanup_test_notes(&pool).await;
}
```

---

## Best Practices

1. ✅ **Tests are isolated** - Cada test limpa dados após execução
2. ✅ **No shared state** - Tests rodam independentemente
3. ✅ **Deterministic** - Sempre mesmo resultado
4. ✅ **Fast** - Completam em < 100ms
5. ✅ **Clear names** - Nome descreve o que testa

---

## Common Issues

### Error: `DATABASE_URL not defined`
**Solução:** Configure a variável de ambiente
```bash
export DATABASE_URL=postgresql://user:password@localhost:5432/notes_db
```

### Error: `connection refused`
**Solução:** Certifique-se que PostgreSQL está rodando
```bash
# Com Docker Compose
docker-compose up -d

# Ou localmente
psql postgres
```

### Tests running slowly
**Solução:** Verifique se há outras conexões abertas ao banco
```bash
# Limpe conexões antigas
docker-compose down
docker-compose up -d
```

---

## CI/CD Integration

Para integrar testes em CI (GitHub Actions, GitLab CI, etc):

```yaml
test:
  script:
    - cargo test --lib
    - cargo test --test integration_tests
```

---

## Next Steps

- [ ] Adicionar testes de performance
- [ ] Adicionar testes de concorrência
- [ ] Implementar property-based testing com `proptest`
- [ ] Adicionar end-to-end tests (E2E)
- [ ] Aumentar coverage para 80%+

---

## Resources

- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [Tokio Testing](https://tokio.rs/tokio/tutorial/select)
