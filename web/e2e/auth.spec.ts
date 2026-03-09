import { test, expect } from "@playwright/test";
import { login } from "./helpers/auth";

test.describe("Authentication", () => {
  test("should login with valid credentials", async ({ page }) => {
    await page.goto("/login");
    await page.getByLabel("Username").fill("admin");
    await page.getByLabel("Password").fill("admin123");
    await page.getByRole("button", { name: "Sign in" }).click();
    await expect(page).toHaveURL(/dashboard/, { timeout: 10000 });
  });

  test("should show error for invalid credentials", async ({ page }) => {
    await page.goto("/login");
    await page.getByLabel("Username").fill("admin");
    await page.getByLabel("Password").fill("wrongpassword");
    await page.getByRole("button", { name: "Sign in" }).click();
    await expect(
      page.getByText("Invalid username or password")
    ).toBeVisible();
  });

  test("should redirect to login when not authenticated", async ({ page }) => {
    await page.goto("/dashboard");
    await expect(page).toHaveURL(/login/);
  });

  test("should logout and redirect to login", async ({ page }) => {
    await login(page);
    // Open user dropdown menu in header
    await page.getByRole("button", { name: /admin/i }).click();
    // Click the Sign out button
    await page.getByRole("button", { name: /sign out/i }).click();
    await expect(page).toHaveURL(/login/);
  });
});
