# Security Checklist

> Use this checklist for security reviews and audits.

---

## Authentication & Authorization

### Authentication
- [ ] JWT tokens have appropriate expiration (access: 15min, refresh: 7days)
- [ ] Refresh token rotation implemented
- [ ] Password requirements enforced (min 8 chars, complexity)
- [ ] Account lockout after failed attempts
- [ ] Secure password reset flow

### Authorization
- [ ] Role-based access control implemented
- [ ] Principle of least privilege followed
- [ ] API endpoints verify user permissions
- [ ] RBAC authorization objects enforce data isolation
- [ ] Admin endpoints protected

---

## Data Protection

### Sensitive Data
- [ ] Passwords hashed with bcrypt/argon2
- [ ] PII encrypted at rest
- [ ] Sensitive data masked in logs
- [ ] No credentials in code or config files
- [ ] Secrets stored in environment variables

### Data Transmission
- [ ] All traffic uses HTTPS/TLS 1.3
- [ ] Certificate pinning (mobile apps)
- [ ] Secure WebSocket connections (WSS)
- [ ] API keys transmitted in headers (not URL)

---

## Input Validation

### General
- [ ] All user input validated and sanitized
- [ ] Whitelist validation preferred
- [ ] Input length limits enforced
- [ ] File upload restrictions (type, size)

### Injection Prevention
- [ ] SQL: Parameterized queries only
- [ ] XSS: Output encoding, CSP headers
- [ ] Command injection: No shell commands with user input
- [ ] Path traversal: Canonicalize paths

---

## API Security

### Rate Limiting
- [ ] Rate limits configured per endpoint
- [ ] Rate limits per user/IP
- [ ] Burst limits for expensive operations
- [ ] 429 responses with Retry-After header

### Request Validation
- [ ] Request size limits
- [ ] Content-Type validation
- [ ] CORS properly configured
- [ ] CSRF protection (if applicable)

---

## Infrastructure

### Network
- [ ] Firewall rules restrict access
- [ ] Internal services not publicly accessible
- [ ] Database not directly accessible from internet
- [ ] VPC/private networking used

### Logging & Monitoring
- [ ] Security events logged
- [ ] Failed auth attempts tracked
- [ ] Anomaly detection configured
- [ ] Logs retained appropriately

---

## Dependency Security

- [ ] Dependencies from trusted sources
- [ ] No known vulnerabilities (pnpm audit, cargo audit, swift package audit, gradle dependencyCheck)
- [ ] Dependency versions pinned
- [ ] Regular dependency updates scheduled
- [ ] License compliance verified

---

## Incident Response

- [ ] Incident response plan documented
- [ ] Contact information up to date
- [ ] Escalation procedures defined
- [ ] Regular security training conducted
