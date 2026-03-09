/**
 * Lightweight monitoring utilities.
 *
 * Sentry integration is intentionally omitted from the frontend bundle to
 * keep the client payload small. Errors are logged to the console and will
 * be captured by the backend Sentry integration when they result in API
 * errors.
 */

export function reportError(
  error: Error,
  context?: Record<string, unknown>
): void {
  console.error("[TasteByte ERP]", error, context);
}
