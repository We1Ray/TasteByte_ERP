# Pre-Deployment Checklist

> Complete this checklist before deploying to production.

---

## Pre-Deployment Verification

### Code Quality
- [ ] All tests pass (`/run-tests`)
- [ ] No linting errors
- [ ] Code review completed and approved
- [ ] All TODO items resolved or tracked

### Security
- [ ] Security audit passed (`/security-audit`)
- [ ] No secrets in codebase
- [ ] Dependencies updated and vulnerability-free
- [ ] RBAC authorization objects tested

### Database
- [ ] Migrations tested in staging
- [ ] Backward compatible schema changes
- [ ] Rollback plan documented
- [ ] Data backup verified

### Configuration
- [ ] Environment variables documented
- [ ] Feature flags configured
- [ ] Rate limits appropriate for production
- [ ] Error tracking configured (Sentry/etc.)

---

## Deployment Steps

### 1. Prepare
- [ ] Notify team of deployment window
- [ ] Create deployment branch/tag
- [ ] Document version number

### 2. Database (if applicable)
- [ ] Run migrations in order
- [ ] Verify migration success
- [ ] Test critical queries

### 3. Backend Services
- [ ] Deploy Rust Backend (port 8000)
- [ ] Deploy Next.js Web (port 3000)
- [ ] Verify health checks pass
- [ ] Check logs for errors

### 4. Mobile Apps
- [ ] Build iOS app (Xcode Archive)
- [ ] Build Android app (Gradle assembleRelease)
- [ ] Verify mobile apps connect to backend
- [ ] Submit to App Store / Google Play for review (if applicable)

### 5. Verification
- [ ] Smoke test critical paths
- [ ] Verify API responses
- [ ] Check monitoring dashboards
- [ ] Confirm alerts are working

---

## Rollback Plan

### Triggers for Rollback
- Error rate > 5%
- P95 latency > 1000ms
- Critical functionality broken
- Data corruption detected

### Rollback Steps
1. [ ] Revert to previous deployment
2. [ ] Rollback database migrations (if safe)
3. [ ] Notify team
4. [ ] Document incident

---

## Post-Deployment

- [ ] Monitor for 30 minutes
- [ ] Check error tracking
- [ ] Verify analytics working
- [ ] Update deployment log
- [ ] Close related tickets/issues
