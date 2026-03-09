# Code Review Checklist

> Run through this checklist before approving any code changes.

---

## General

- [ ] Code follows project conventions (see `memory-bank/patterns.md`)
- [ ] No hardcoded secrets, API keys, or credentials
- [ ] No TODO/FIXME comments left unaddressed
- [ ] Changes are minimal and focused (no scope creep)
- [ ] No unnecessary console.log/print/dbg! statements

---

## Security

- [ ] User input is validated at API boundaries
- [ ] SQL queries use parameterized statements (no string concatenation)
- [ ] Authentication/authorization checks are in place
- [ ] Sensitive data is not logged
- [ ] RBAC authorization objects applied for new endpoints
- [ ] No SQL injection vulnerabilities
- [ ] No XSS vulnerabilities (for frontend)

---

## iOS (Swift/SwiftUI) Specific

- [ ] Uses `@StateObject` for owned ViewModels, `@ObservedObject` for passed-in
- [ ] Proper async/await error handling with `do/catch`
- [ ] ViewModels marked with `@MainActor` for thread safety
- [ ] No retain cycles (use `[weak self]` in closures)
- [ ] Accessibility modifiers applied (`.accessibilityLabel`, `.accessibilityHint`)
- [ ] Responsive design with `GeometryReader` or adaptive layouts

## Android (Kotlin/Jetpack Compose) Specific

- [ ] Uses `collectAsStateWithLifecycle()` for StateFlow in Compose
- [ ] Proper coroutine scope management (viewModelScope)
- [ ] No blocking calls on main thread
- [ ] Hilt dependency injection configured correctly
- [ ] Accessibility (contentDescription, semantics)
- [ ] Responsive design with `WindowSizeClass`

---

## Rust Specific

- [ ] Proper error handling with `Result<T, E>`
- [ ] No `unwrap()` in production code (use `?` or proper error handling)
- [ ] Async functions are truly async (no blocking in async context)
- [ ] Resources are properly cleaned up (Drop trait)
- [ ] Clippy warnings addressed

---

## Next.js Specific

- [ ] Server Components used for data fetching (no unnecessary `"use client"`)
- [ ] Client Components only where interactivity is needed
- [ ] Tanstack Query for data fetching patterns
- [ ] Zod schemas for form validation
- [ ] Loading and error boundaries provided
- [ ] No sensitive data in client bundles

---

## SQL Specific

- [ ] Table follows naming conventions (module_prefix + snake_case)
- [ ] Primary key is UUID with default
- [ ] Foreign keys have appropriate ON DELETE action
- [ ] Indexes on frequently queried columns and FKs
- [ ] `created_at` and `updated_at` columns present
- [ ] `updated_at` trigger applied

---

## Testing

- [ ] New code has corresponding tests
- [ ] Edge cases are covered
- [ ] Tests are deterministic (no flaky tests)
- [ ] Mock external dependencies

---

## Documentation

- [ ] Complex logic has inline comments
- [ ] Public APIs have documentation
- [ ] Breaking changes documented
- [ ] `memory-bank/decisions.md` updated for architectural decisions
