# Authentication Guide

## Overview
A API implementa **JWT (JSON Web Tokens)** para autenticação. Todos os endpoints protegidos requerem um token válido no header `Authorization`.

---

## How It Works

### 1. Login (Get Token)
```bash
POST /login
Content-Type: application/json

{
  "username": "user123",
  "password": "securepass123"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### 2. Use Token (Access Protected Endpoints)
```bash
GET /notes
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### 3. Token Expiration
- **Duration:** 24 horas
- **After expiration:** Faça login novamente para obter novo token

---

## Login Endpoint

### Request
```http
POST /login
Content-Type: application/json

{
  "username": "myuser",
  "password": "mypassword"
}
```

### Validation Rules
| Field | Min | Max | Required |
|-------|-----|-----|----------|
| username | 3 | 50 | Yes |
| password | 8 | 128 | Yes |

### Success Response
```json
HTTP/1.1 200 OK
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjoxLCJleHAiOjE3MTc2Nzg5OTksImlhdCI6MTcxNzU5MjU5OX0.xyz..."
}
```

### Error Response
```json
HTTP/1.1 400 Bad Request
{
  "error": "Content must be between 8 and 128 characters"
}
```

---

## Using the Token

### Header Format
```
Authorization: Bearer <token>
```

### Example with curl
```bash
TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/notes
```

### Example with JavaScript/fetch
```javascript
const token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";

fetch("http://localhost:3000/notes", {
  method: "GET",
  headers: {
    "Authorization": `Bearer ${token}`
  }
})
.then(response => response.json())
.then(data => console.log(data));
```

### Example with Python/requests
```python
import requests

token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
headers = {"Authorization": f"Bearer {token}"}

response = requests.get(
  "http://localhost:3000/notes",
  headers=headers
)
print(response.json())
```

---

## Token Structure

JWT tokens têm 3 partes separadas por `.`:
```
header.payload.signature
```

### Example Decoded
```json
// Header
{
  "alg": "HS256",
  "typ": "JWT"
}

// Payload
{
  "user_id": 1,
  "exp": 1717678999,      // Unix timestamp da expiração
  "iat": 1717592599       // Unix timestamp de criação
}

// Signature (verificado pelo servidor)
```

---

## Step-by-Step Usage

### 1. Get a Token
```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username": "user123", "password": "password123"}'
```

Response:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjoxLCJleHAiOjE3MTc2Nzg5OTksImlhdCI6MTcxNzU5MjU5OX0.xyz..."
}
```

### 2. Copy the Token
```bash
TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

### 3. List Notes with Token
```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/notes
```

### 4. Create Note with Token
```bash
curl -X POST http://localhost:3000/notes \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "My new note"}'
```

### 5. Update Note with Token
```bash
curl -X PATCH http://localhost:3000/notes/1 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "Updated note"}'
```

### 6. Delete Note with Token
```bash
curl -X DELETE http://localhost:3000/notes/1 \
  -H "Authorization: Bearer $TOKEN"
```

---

## Error Handling

### Missing Token
```bash
curl http://localhost:3000/notes
```

**Response:**
```json
HTTP/1.1 401 Unauthorized
{
  "error": "unauthorized - invalid or missing token"
}
```

### Invalid Token
```bash
curl -H "Authorization: Bearer invalid_token_xyz" \
  http://localhost:3000/notes
```

**Response:**
```json
HTTP/1.1 401 Unauthorized
{
  "error": "unauthorized - invalid or missing token"
}
```

### Expired Token
```bash
# Token criado há 25 horas atrás (expirou)
curl -H "Authorization: Bearer expired_token" \
  http://localhost:3000/notes
```

**Response:**
```json
HTTP/1.1 401 Unauthorized
{
  "error": "unauthorized - invalid or missing token"
}
```

### Wrong Header Format
```bash
curl -H "Authorization: $TOKEN" \
  http://localhost:3000/notes
```

**Response:**
```json
HTTP/1.1 401 Unauthorized
{
  "error": "unauthorized - invalid or missing token"
}
```

⚠️ **Note:** Deve ser `Bearer <token>`, não só o token

---

## Security Notes

### Current Implementation (Development)
⚠️ **NOT FOR PRODUCTION**
- JWT secret é hardcoded: `your-secret-key-change-in-production`
- Aceita qualquer username/password
- Sem validação contra banco de dados

### For Production
✅ **You MUST:**

1. **Change JWT Secret**
   ```rust
   // src/auth.rs
   const JWT_SECRET: &[u8] = b"your-production-secret-key-min-32-chars-xyz";
   ```

2. **Validate Against Database**
   ```rust
   // Instead of accepting any login:
   pub async fn login(...) {
       // Lookup user by username in database
       // Hash password with bcrypt
       // Verify password matches stored hash
   }
   ```

3. **Use HTTPS**
   - Never transmit tokens over HTTP
   - Token contains user_id and should be encrypted in transit

4. **Store Tokens Securely**
   - Frontend: Use HttpOnly cookies (not localStorage)
   - Backend: Use secure sessions

5. **Implement Refresh Tokens**
   - Short-lived access tokens (15 min)
   - Long-lived refresh tokens (7 days)
   - Rotate tokens periodically

6. **Rate Limiting on Login**
   - Prevent brute force attacks
   - Max 5 login attempts per IP per hour

---

## Testing Authentication

### Unit Tests
```bash
cargo test auth
```

Tests implemented:
- ✅ `test_create_jwt` - Token creation
- ✅ `test_hash_password` - Password hashing
- ✅ `test_verify_password_invalid` - Wrong password rejection

### Manual Testing with curl
```bash
# 1. Login
TOKEN=$(curl -s -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username": "user123", "password": "password123"}' | jq -r '.token')

echo "Token: $TOKEN"

# 2. Use token
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/notes

# 3. Test expiration (if you modify token, it will fail)
curl -H "Authorization: Bearer ${TOKEN}xxx" \
  http://localhost:3000/notes
```

---

## Logging

Todos os eventos de autenticação são logados:

```
[INFO] login attempt, username=user123
[INFO] login successful, username=user123
[ERROR] validation error: ...
```

Enable detailed logging:
```bash
RUST_LOG=debug cargo run
```

---

## Common Issues

### "unauthorized - invalid or missing token"
1. ✅ Verifique se está usando `Bearer <token>` (com espaço)
2. ✅ Verifique se o token não expirou
3. ✅ Verifique se o token está correto (não cortado)
4. ✅ Verifique se está enviando no header `Authorization`

### "validation error"
1. ✅ Username deve ter 3-50 caracteres
2. ✅ Password deve ter 8-128 caracteres

### Token not returning
1. ✅ Verify login endpoint exists: `POST /login`
2. ✅ Check request format (JSON, correct fields)
3. ✅ Check server is running: `cargo run`

---

## Future Enhancements

- [ ] Database user storage with password hashing
- [ ] Email verification
- [ ] Password reset flow
- [ ] Refresh tokens
- [ ] Rate limiting on login
- [ ] Multi-factor authentication (MFA)
- [ ] Role-based access control (RBAC)
- [ ] Token revocation / logout
- [ ] API key authentication

---

## Resources

- [JWT.io - JWT Debugger](https://jwt.io) - Decode and verify tokens
- [jsonwebtoken Crate](https://docs.rs/jsonwebtoken)
- [bcrypt Crate](https://docs.rs/bcrypt)
- [RFC 7519 - JSON Web Token (JWT)](https://tools.ietf.org/html/rfc7519)
