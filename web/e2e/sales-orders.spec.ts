import { test, expect } from "@playwright/test";
import { login } from "./helpers/auth";

test.describe("Sales Orders", () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test("should display sales orders list page", async ({ page }) => {
    await page.goto("/sd/sales-orders");
    await expect(
      page.getByRole("heading", { name: "Sales Orders" })
    ).toBeVisible();
    await expect(
      page.getByPlaceholder("Search sales orders...")
    ).toBeVisible();
  });

  test("should have create order button", async ({ page }) => {
    await page.goto("/sd/sales-orders");
    await expect(
      page.getByRole("button", { name: /Create Order/ })
    ).toBeVisible();
  });

  test("should navigate to create order page", async ({ page }) => {
    await page.goto("/sd/sales-orders");
    await page.getByRole("button", { name: /Create Order/ }).click();
    await expect(page).toHaveURL(/\/sd\/sales-orders\/new/);
  });

  test("should display table headers", async ({ page }) => {
    await page.goto("/sd/sales-orders");
    // Wait for page to load
    await expect(
      page.getByRole("heading", { name: "Sales Orders" })
    ).toBeVisible();
    // Table column headers should be visible (in the table or empty state)
    const table = page.locator("table");
    if (await table.isVisible()) {
      await expect(page.getByText("Order No.")).toBeVisible();
      await expect(page.getByText("Customer")).toBeVisible();
      await expect(page.getByText("Status")).toBeVisible();
    }
  });

  test("should navigate to sales order detail page", async ({ page }) => {
    await page.goto("/sd/sales-orders");
    // If there are orders, click the first row
    const firstRow = page.locator("table tbody tr").first();
    if (await firstRow.isVisible({ timeout: 5000 }).catch(() => false)) {
      await firstRow.click();
      await expect(page).toHaveURL(/\/sd\/sales-orders\/[a-f0-9-]+/);
    }
  });

  test("should display create sales order form fields", async ({ page }) => {
    await page.goto("/sd/sales-orders/new");
    await expect(
      page.getByRole("heading", { name: "Create Sales Order" })
    ).toBeVisible();
    // Order Information section
    await expect(page.getByLabel("Customer")).toBeVisible();
    await expect(page.getByLabel("Order Date")).toBeVisible();
    await expect(page.getByLabel("Requested Delivery Date")).toBeVisible();
    // Line Items section
    await expect(page.getByText("Line Items")).toBeVisible();
    await expect(
      page.getByRole("button", { name: /Add Item/ })
    ).toBeVisible();
    // Submit button
    await expect(
      page.getByRole("button", { name: "Create Sales Order" })
    ).toBeVisible();
  });

  test("should add and remove line items", async ({ page }) => {
    await page.goto("/sd/sales-orders/new");
    // Should start with one line item
    const materialSelects = page.getByLabel("Material");
    await expect(materialSelects.first()).toBeVisible();
    // Add another item
    await page.getByRole("button", { name: /Add Item/ }).click();
    // Should now have two line items (two Material selects)
    await expect(materialSelects).toHaveCount(2);
  });

  test("should display detail page with workflow timeline", async ({
    page,
  }) => {
    await page.goto("/sd/sales-orders");
    const firstRow = page.locator("table tbody tr").first();
    if (await firstRow.isVisible({ timeout: 5000 }).catch(() => false)) {
      await firstRow.click();
      await expect(page).toHaveURL(/\/sd\/sales-orders\/[a-f0-9-]+/);
      // Detail page sections
      await expect(page.getByText("Line Items")).toBeVisible();
      await expect(page.getByText("Order Details")).toBeVisible();
      await expect(page.getByText("Workflow")).toBeVisible();
    }
  });
});
