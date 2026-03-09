import { test, expect } from "@playwright/test";
import { login } from "./helpers/auth";

test.describe("Dashboard", () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test("should display dashboard page header", async ({ page }) => {
    await expect(page.getByRole("heading", { name: "Dashboard" })).toBeVisible();
  });

  test("should display KPI cards", async ({ page }) => {
    await expect(page.getByText("Total Revenue")).toBeVisible();
    await expect(page.getByText("Sales Orders")).toBeVisible();
    await expect(page.getByText("Inventory Qty")).toBeVisible();
    await expect(page.getByText("Pending Production")).toBeVisible();
  });

  test("should display AR/AP summary cards", async ({ page }) => {
    await expect(page.getByText("Open AR (Receivables)")).toBeVisible();
    await expect(page.getByText("Open AP (Payables)")).toBeVisible();
  });

  test("should display charts section", async ({ page }) => {
    await expect(
      page.getByText("Revenue vs Costs (Last 6 Months)")
    ).toBeVisible();
    await expect(
      page.getByText("Order Trends (Last 6 Months)")
    ).toBeVisible();
  });

  test("should display quick actions", async ({ page }) => {
    await expect(page.getByText("Quick Actions")).toBeVisible();
    await expect(page.getByText("Create Sales Order")).toBeVisible();
    await expect(page.getByText("Create Purchase Order")).toBeVisible();
    await expect(page.getByText("Add Material")).toBeVisible();
    await expect(page.getByText("New Journal Entry")).toBeVisible();
  });

  test("should display recent sales orders table", async ({ page }) => {
    await expect(page.getByText("Recent Sales Orders")).toBeVisible();
  });
});
