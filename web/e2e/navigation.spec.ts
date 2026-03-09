import { test, expect } from "@playwright/test";
import { login } from "./helpers/auth";

test.describe("Navigation", () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test("should navigate to Financial Accounting via sidebar", async ({
    page,
  }) => {
    await page.getByRole("link", { name: /FI - Financial Accounting/ }).click();
    await expect(page).toHaveURL(/\/fi\//);
  });

  test("should navigate to Materials Management via sidebar", async ({
    page,
  }) => {
    await page
      .getByRole("link", { name: /MM - Materials Management/ })
      .click();
    await expect(page).toHaveURL(/\/mm\//);
  });

  test("should navigate to Sales & Distribution via sidebar", async ({
    page,
  }) => {
    await page
      .getByRole("link", { name: /SD - Sales & Distribution/ })
      .click();
    await expect(page).toHaveURL(/\/sd\//);
  });

  test("should navigate to Production Planning via sidebar", async ({
    page,
  }) => {
    await page
      .getByRole("link", { name: /PP - Production Planning/ })
      .click();
    await expect(page).toHaveURL(/\/pp\//);
  });

  test("should navigate to Human Resources via sidebar", async ({ page }) => {
    await page.getByRole("link", { name: /HR - Human Resources/ }).click();
    await expect(page).toHaveURL(/\/hr\//);
  });

  test("should navigate back to dashboard", async ({ page }) => {
    await page
      .getByRole("link", { name: /MM - Materials Management/ })
      .click();
    await expect(page).toHaveURL(/\/mm\//);
    await page.getByRole("link", { name: "Dashboard" }).click();
    await expect(page).toHaveURL(/\/dashboard/);
  });

  test("should show sidebar module children when active", async ({ page }) => {
    await page
      .getByRole("link", { name: /MM - Materials Management/ })
      .click();
    await expect(page.getByRole("link", { name: "Materials" })).toBeVisible();
    await expect(
      page.getByRole("link", { name: "Purchase Orders" })
    ).toBeVisible();
    await expect(
      page.getByRole("link", { name: "Stock Overview" })
    ).toBeVisible();
  });

  test("should display breadcrumb on module pages", async ({ page }) => {
    await page.goto("/mm/materials");
    // Breadcrumb nav should show: Home > Materials Management > Materials
    const breadcrumb = page.locator("nav").filter({ hasText: "Materials Management" });
    await expect(breadcrumb).toBeVisible();
    await expect(breadcrumb.getByText("Materials Management")).toBeVisible();
    await expect(breadcrumb.getByText("Materials")).toBeVisible();
  });

  test("should navigate via breadcrumb links", async ({ page }) => {
    await page.goto("/mm/materials");
    // Click the "Materials Management" breadcrumb link to go to /mm
    const breadcrumb = page.locator("nav").filter({ hasText: "Materials Management" });
    await breadcrumb.getByRole("link", { name: "Materials Management" }).click();
    await expect(page).toHaveURL(/\/mm$/);
  });

  test("should load module pages without errors", async ({ page }) => {
    const modulePaths = [
      "/fi/accounts",
      "/mm/materials",
      "/sd/customers",
      "/pp/boms",
      "/hr/employees",
    ];
    for (const path of modulePaths) {
      await page.goto(path);
      // Each page should have a heading (PageHeader renders an h1)
      await expect(page.locator("h1, h2").first()).toBeVisible();
    }
  });
});
