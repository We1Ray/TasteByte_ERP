import { chromium, Page, Browser } from "@playwright/test";
import * as path from "path";
import * as fs from "fs";

const BASE_URL = "http://localhost:3000";
const SCREENSHOT_DIR = path.join(__dirname, "..", "docs", "screenshots");
const VIDEO_DIR = path.join(__dirname, "..", "docs", "videos");

async function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function login(page: Page) {
  await page.goto(`${BASE_URL}/login`);
  await page.waitForLoadState("networkidle");
  await sleep(1000);
  await page.fill('input[name="username"]', "admin");
  await page.fill('input[name="password"]', "admin123");
  await page.click('button[type="submit"]');
  await page.waitForURL("**/dashboard", { timeout: 15000 });
  await page.waitForLoadState("networkidle");
  await sleep(2000);
}

async function takeScreenshot(page: Page, name: string) {
  const filePath = path.join(SCREENSHOT_DIR, `${name}.png`);
  await page.screenshot({ path: filePath, fullPage: false });
  console.log(`  ✅ ${name}.png`);
}

async function main() {
  fs.mkdirSync(SCREENSHOT_DIR, { recursive: true });
  fs.mkdirSync(VIDEO_DIR, { recursive: true });

  const browser = await chromium.launch({ headless: true });

  // ── 1. Login Page Screenshot ──
  console.log("📸 Taking screenshots...");
  const loginCtx = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
  });
  const loginPage = await loginCtx.newPage();
  await loginPage.goto(`${BASE_URL}/login`);
  await loginPage.waitForLoadState("networkidle");
  await sleep(1500);
  await takeScreenshot(loginPage, "01-login");
  await loginCtx.close();

  // ── 2. Main app screenshots ──
  const ctx = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
  });
  const page = await ctx.newPage();
  await login(page);

  // Dashboard
  await takeScreenshot(page, "02-dashboard");

  // FI - Chart of Accounts
  await page.goto(`${BASE_URL}/fi/accounts`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "03-fi-accounts");

  // FI - Journal Entries
  await page.goto(`${BASE_URL}/fi/journal`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "04-fi-journal");

  // FI - Reports
  await page.goto(`${BASE_URL}/fi/reports`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "05-fi-reports");

  // MM - Materials
  await page.goto(`${BASE_URL}/mm/materials`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "06-mm-materials");

  // MM - Purchase Orders
  await page.goto(`${BASE_URL}/mm/purchase-orders`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "07-mm-purchase-orders");

  // MM - Stock
  await page.goto(`${BASE_URL}/mm/stock`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "08-mm-stock");

  // SD - Customers
  await page.goto(`${BASE_URL}/sd/customers`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "09-sd-customers");

  // SD - Sales Orders
  await page.goto(`${BASE_URL}/sd/sales-orders`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "10-sd-sales-orders");

  // SD - Invoices
  await page.goto(`${BASE_URL}/sd/invoices`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "11-sd-invoices");

  // PP - BOMs
  await page.goto(`${BASE_URL}/pp/boms`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "12-pp-boms");

  // PP - Production Orders
  await page.goto(`${BASE_URL}/pp/production-orders`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "13-pp-production-orders");

  // HR - Employees
  await page.goto(`${BASE_URL}/hr/employees`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "14-hr-employees");

  // HR - Attendance
  await page.goto(`${BASE_URL}/hr/attendance`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "15-hr-attendance");

  // WM
  await page.goto(`${BASE_URL}/wm`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "16-wm-warehouse");

  // QM
  await page.goto(`${BASE_URL}/qm`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "17-qm-quality");

  // CO
  await page.goto(`${BASE_URL}/co`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "18-co-controlling");

  // Admin - Users
  await page.goto(`${BASE_URL}/admin/users`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "19-admin-users");

  // Admin - Roles
  await page.goto(`${BASE_URL}/admin/roles`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "20-admin-roles");

  // Developer
  await page.goto(`${BASE_URL}/developer`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "21-developer-hub");

  // Developer - Operations
  await page.goto(`${BASE_URL}/developer/operations`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "22-developer-operations");

  // Lowcode
  await page.goto(`${BASE_URL}/lowcode`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "23-lowcode-platform");

  // Notifications
  await page.goto(`${BASE_URL}/notifications`);
  await page.waitForLoadState("networkidle");
  await sleep(2000);
  await takeScreenshot(page, "24-notifications");

  await ctx.close();

  // ── 3. Record GIF-worthy video flows ──
  console.log("\n🎬 Recording operation videos...");

  // --- GIF 1: Login flow ---
  const gifCtx1 = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
    recordVideo: { dir: VIDEO_DIR, size: { width: 1440, height: 900 } },
  });
  const gifPage1 = await gifCtx1.newPage();
  await gifPage1.goto(`${BASE_URL}/login`);
  await gifPage1.waitForLoadState("networkidle");
  await sleep(1500);
  await gifPage1.fill('input[name="username"]', "admin");
  await sleep(500);
  await gifPage1.fill('input[name="password"]', "admin123");
  await sleep(500);
  await gifPage1.click('button[type="submit"]');
  await gifPage1.waitForURL("**/dashboard", { timeout: 15000 });
  await sleep(3000);
  await gifPage1.close();
  const video1 = gifPage1.video();
  if (video1) {
    const videoPath = await video1.path();
    fs.renameSync(videoPath, path.join(VIDEO_DIR, "login-flow.webm"));
    console.log("  ✅ login-flow.webm");
  }
  await gifCtx1.close();

  // --- GIF 2: Navigation tour ---
  const gifCtx2 = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
    recordVideo: { dir: VIDEO_DIR, size: { width: 1440, height: 900 } },
  });
  const gifPage2 = await gifCtx2.newPage();
  await login(gifPage2);
  await sleep(1500);

  // Tour modules
  const tourPages = [
    "/fi/accounts",
    "/mm/materials",
    "/sd/sales-orders",
    "/pp/production-orders",
    "/hr/employees",
    "/wm",
    "/qm",
    "/co",
  ];
  for (const p of tourPages) {
    await gifPage2.goto(`${BASE_URL}${p}`);
    await gifPage2.waitForLoadState("networkidle");
    await sleep(2000);
  }
  await gifPage2.close();
  const video2 = gifPage2.video();
  if (video2) {
    const videoPath = await video2.path();
    fs.renameSync(videoPath, path.join(VIDEO_DIR, "module-tour.webm"));
    console.log("  ✅ module-tour.webm");
  }
  await gifCtx2.close();

  // --- GIF 3: Create a new journal entry ---
  const gifCtx3 = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
    recordVideo: { dir: VIDEO_DIR, size: { width: 1440, height: 900 } },
  });
  const gifPage3 = await gifCtx3.newPage();
  await login(gifPage3);
  await gifPage3.goto(`${BASE_URL}/fi/journal`);
  await gifPage3.waitForLoadState("networkidle");
  await sleep(2000);

  // Click new journal entry if button exists
  const newBtn = gifPage3.locator('a[href="/fi/journal/new"], button:has-text("新增"), button:has-text("New"), button:has-text("建立")');
  if (await newBtn.count() > 0) {
    await newBtn.first().click();
    await gifPage3.waitForLoadState("networkidle");
    await sleep(3000);
  }
  await gifPage3.close();
  const video3 = gifPage3.video();
  if (video3) {
    const videoPath = await video3.path();
    fs.renameSync(videoPath, path.join(VIDEO_DIR, "create-journal.webm"));
    console.log("  ✅ create-journal.webm");
  }
  await gifCtx3.close();

  // --- GIF 4: MM Material management ---
  const gifCtx4 = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
    recordVideo: { dir: VIDEO_DIR, size: { width: 1440, height: 900 } },
  });
  const gifPage4 = await gifCtx4.newPage();
  await login(gifPage4);
  await gifPage4.goto(`${BASE_URL}/mm/materials`);
  await gifPage4.waitForLoadState("networkidle");
  await sleep(2000);

  // Try clicking first material row
  const firstRow = gifPage4.locator("table tbody tr").first();
  if (await firstRow.count() > 0) {
    await firstRow.click();
    await gifPage4.waitForLoadState("networkidle");
    await sleep(3000);
  }
  await gifPage4.close();
  const video4 = gifPage4.video();
  if (video4) {
    const videoPath = await video4.path();
    fs.renameSync(videoPath, path.join(VIDEO_DIR, "material-detail.webm"));
    console.log("  ✅ material-detail.webm");
  }
  await gifCtx4.close();

  // --- GIF 5: SD Sales Order workflow ---
  const gifCtx5 = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
    recordVideo: { dir: VIDEO_DIR, size: { width: 1440, height: 900 } },
  });
  const gifPage5 = await gifCtx5.newPage();
  await login(gifPage5);
  await gifPage5.goto(`${BASE_URL}/sd/sales-orders`);
  await gifPage5.waitForLoadState("networkidle");
  await sleep(2000);

  const soRow = gifPage5.locator("table tbody tr").first();
  if (await soRow.count() > 0) {
    await soRow.click();
    await gifPage5.waitForLoadState("networkidle");
    await sleep(3000);
  }
  await gifPage5.close();
  const video5 = gifPage5.video();
  if (video5) {
    const videoPath = await video5.path();
    fs.renameSync(videoPath, path.join(VIDEO_DIR, "sales-order-flow.webm"));
    console.log("  ✅ sales-order-flow.webm");
  }
  await gifCtx5.close();

  await browser.close();
  console.log("\n✨ All screenshots and videos captured!");
  console.log(`📁 Screenshots: ${SCREENSHOT_DIR}`);
  console.log(`📁 Videos: ${VIDEO_DIR}`);
}

main().catch(console.error);
