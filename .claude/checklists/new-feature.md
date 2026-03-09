# New Feature Checklist

> Use this checklist when implementing a new feature.

---

## Planning Phase

- [ ] Requirements clearly defined
- [ ] User stories documented
- [ ] Technical approach decided
- [ ] Dependencies identified
- [ ] Estimated complexity assessed
- [ ] Decision recorded in `memory-bank/decisions.md` (if architectural)

---

## Design Phase

### Architecture
- [ ] Fits within existing architecture
- [ ] No unnecessary complexity
- [ ] Follows separation of concerns
- [ ] Backward compatible (or migration plan)

### Database (if applicable)
- [ ] Schema designed
- [ ] Indexes planned
- [ ] RBAC authorization objects designed
- [ ] Migration file numbered correctly

### API (if applicable)
- [ ] Endpoints designed
- [ ] Request/Response models defined
- [ ] Error codes documented
- [ ] Rate limits considered

---

## Implementation Phase

### Before Coding
- [ ] Branch created from main
- [ ] Reviewed existing code patterns (`memory-bank/patterns.md`)
- [ ] Checked templates (`templates/`)

### During Coding
- [ ] Following project conventions
- [ ] Writing tests alongside code
- [ ] No hardcoded values
- [ ] Error handling implemented
- [ ] Logging added for debugging

### After Coding
- [ ] Self-reviewed code
- [ ] Ran linters and formatters
- [ ] All tests pass
- [ ] No console.log/print/dbg! statements left
- [ ] Cleaned up debug code

---

## Layer-Specific Checklists

### Rust Backend
- [ ] Handler with validation (validator crate)
- [ ] Repository pattern for database operations
- [ ] Error types defined (AppError variants)
- [ ] Proper async handling
- [ ] Routes registered in module router
- [ ] Authorization object checks applied

### Next.js Web
- [ ] Server Components for data fetching
- [ ] Client Components only where needed (interactivity)
- [ ] Tanstack Query for data management
- [ ] Zod schema for form validation
- [ ] Loading and error boundaries
- [ ] Responsive design with Tailwind

### iOS Mobile (Swift/SwiftUI)
- [ ] SwiftUI View with @StateObject ViewModel
- [ ] Repository pattern followed
- [ ] Error states handled in UI
- [ ] Loading states shown (ProgressView)
- [ ] Offline support considered
- [ ] Responsive design tested across device sizes

### Android Mobile (Kotlin/Jetpack Compose)
- [ ] Composable with ViewModel + StateFlow
- [ ] Repository pattern followed
- [ ] Error states handled in UI
- [ ] Loading states shown (CircularProgressIndicator)
- [ ] Offline support considered
- [ ] Responsive design tested across device sizes

### SQL
- [ ] Table with proper constraints and module prefix
- [ ] Indexes on query columns
- [ ] Trigger for updated_at
- [ ] Comments added

---

## Testing Phase

- [ ] Unit tests written
- [ ] Integration tests (if applicable)
- [ ] Edge cases covered
- [ ] Error scenarios tested
- [ ] Performance acceptable

---

## Documentation Phase

- [ ] Code comments for complex logic
- [ ] API documentation updated
- [ ] README updated (if needed)
- [ ] Learnings recorded in `memory-bank/learnings.md`

---

## Review Phase

- [ ] Code review checklist passed (`checklists/code-review.md`)
- [ ] Security checklist items addressed
- [ ] PR description complete
- [ ] Screenshots/videos (if UI changes)

---

## Completion

- [ ] Merged to main
- [ ] Deployed to staging
- [ ] QA verified
- [ ] Ready for production
