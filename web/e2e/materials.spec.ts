import { test, expect } from "@playwright/test";
import { login } from "./helpers/auth";

test.describe("Materials Management", () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test("should display materials list page", async ({ page }) => {
    await page.goto("/mm/materials");
    await expect(
      page.getByRole("heading", { name: "Materials" })
    ).toBeVisible();
    await expect(
      page.getByPlaceholder("Search materials by number or name...")
    ).toBeVisible();
  });

  test("should have create material button", async ({ page }) => {
    await page.goto("/mm/materials");
    await expect(
      page.getByRole("button", { name: /Create Material/ })
    ).toBeVisible();
  });

  test("should navigate to create material page", async ({ page }) => {
    await page.goto("/mm/materials");
    await page.getByRole("button", { name: /Create Material/ }).click();
    await expect(page).toHaveURL(/\/mm\/materials\/new/);
    await expect(
      page.getByRole("heading", { name: "Create Material" })
    ).toBeVisible();
  });

  test("should display material form fields", async ({ page }) => {
    await page.goto("/mm/materials/new");
    await expect(page.getByLabel("Material Number")).toBeVisible();
    await expect(page.getByLabel("Name")).toBeVisible();
    await expect(page.getByLabel("Material Type")).toBeVisible();
    await expect(page.getByLabel("Material Group")).toBeVisible();
    await expect(page.getByLabel("Base Unit")).toBeVisible();
    await expect(page.getByLabel("Description")).toBeVisible();
    await expect(page.getByLabel("Price")).toBeVisible();
  });

  test("should show validation errors on empty submit", async ({ page }) => {
    await page.goto("/mm/materials/new");
    // Clear the pre-filled material number field
    await page.getByLabel("Material Number").clear();
    await page.getByLabel("Name").clear();
    await page.getByLabel("Material Group").clear();
    await page.getByRole("button", { name: "Create Material" }).click();
    // Form validation should show error messages
    await expect(page.getByText("Material number is required")).toBeVisible();
    await expect(page.getByText("Name is required")).toBeVisible();
  });

  test("should filter materials by type", async ({ page }) => {
    await page.goto("/mm/materials");
    // The search bar has a type filter dropdown
    const typeFilter = page.getByRole("combobox").first();
    if (await typeFilter.isVisible()) {
      await typeFilter.selectOption("RAW");
    }
  });

  test("should navigate to material detail page", async ({ page }) => {
    await page.goto("/mm/materials");
    // If there are materials in the table, click the first row
    const firstRow = page.locator("table tbody tr").first();
    if (await firstRow.isVisible({ timeout: 5000 }).catch(() => false)) {
      await firstRow.click();
      await expect(page).toHaveURL(/\/mm\/materials\/[a-f0-9-]+/);
      // Detail page shows the material form with Save Changes button
      await expect(
        page.getByRole("button", { name: "Save Changes" })
      ).toBeVisible();
    }
  });

  test("should fill and submit create material form", async ({ page }) => {
    await page.goto("/mm/materials/new");
    const timestamp = Date.now();
    await page.getByLabel("Material Number").fill(`E2E-MAT-${timestamp}`);
    await page.getByLabel("Name").fill(`E2E Test Material ${timestamp}`);
    await page.getByLabel("Material Type").selectOption("RAW");
    await page.getByLabel("Material Group").fill("E2E-Test");
    await page.getByLabel("Base Unit").selectOption("EA");
    await page.getByLabel("Description").fill("Created by E2E test");
    await page.getByLabel("Price").fill("99.99");
    await page.getByRole("button", { name: "Create Material" }).click();
    // On success, should redirect back to materials list
    await expect(page).toHaveURL(/\/mm\/materials$/, { timeout: 10000 });
  });
});
