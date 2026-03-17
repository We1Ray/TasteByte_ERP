import { chromium } from "@playwright/test";
import * as path from "path";
import * as fs from "fs";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const BASE_URL = "http://localhost:3000";
const SCREENSHOT_DIR = path.join(__dirname, "..", "docs", "screenshots");
const VIDEO_DIR = path.join(__dirname, "..", "docs", "videos");

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function login(page) {
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

async function shot(page, name) {
  const filePath = path.join(SCREENSHOT_DIR, `${name}.png`);
  await page.screenshot({ path: filePath, fullPage: false });
  console.log(`  ✅ ${name}.png`);
}

async function main() {
  fs.mkdirSync(SCREENSHOT_DIR, { recursive: true });
  fs.mkdirSync(VIDEO_DIR, { recursive: true });

  const browser = await chromium.launch({ headless: true });

  // ── 1. Login Page ──
  console.log("📸 Taking screenshots...");
  let ctx = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "zh-TW" });
  let page = await ctx.newPage();
  await page.goto(`${BASE_URL}/login`);
  await page.waitForLoadState("networkidle");
  await sleep(1500);
  await shot(page, "01-login");
  await ctx.close();

  // ── 2. All module pages ──
  ctx = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "zh-TW" });
  page = await ctx.newPage();
  await login(page);
  await shot(page, "02-dashboard");

  const pages = [
    ["fi/accounts", "03-fi-accounts"],
    ["fi/journal", "04-fi-journal"],
    ["fi/reports", "05-fi-reports"],
    ["mm/materials", "06-mm-materials"],
    ["mm/purchase-orders", "07-mm-purchase-orders"],
    ["mm/stock", "08-mm-stock"],
    ["mm/reports", "09-mm-reports"],
    ["sd/customers", "10-sd-customers"],
    ["sd/sales-orders", "11-sd-sales-orders"],
    ["sd/invoices", "12-sd-invoices"],
    ["sd/reports", "13-sd-reports"],
    ["pp/boms", "14-pp-boms"],
    ["pp/production-orders", "15-pp-production-orders"],
    ["hr/employees", "16-hr-employees"],
    ["hr/attendance", "17-hr-attendance"],
    ["wm", "18-wm-warehouse"],
    ["qm", "19-qm-quality"],
    ["co", "20-co-controlling"],
    ["admin/users", "21-admin-users"],
    ["admin/roles", "22-admin-roles"],
    ["developer", "23-developer-hub"],
    ["developer/operations", "24-developer-operations"],
    ["lowcode", "25-lowcode-platform"],
    ["notifications", "26-notifications"],
  ];

  for (const [url, name] of pages) {
    await page.goto(`${BASE_URL}/${url}`);
    await page.waitForLoadState("networkidle");
    await sleep(2000);
    await shot(page, name);
  }
  await ctx.close();

  // ── 3. Record videos for GIF conversion ──
  console.log("\n🎬 Recording operation videos...");

  // GIF 1: Login flow
  let gifCtx = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
    recordVideo: { dir: VIDEO_DIR, size: { width: 1440, height: 900 } },
  });
  let gifPage = await gifCtx.newPage();
  await gifPage.goto(`${BASE_URL}/login`);
  await gifPage.waitForLoadState("networkidle");
  await sleep(1500);
  await gifPage.fill('input[name="username"]', "admin");
  await sleep(500);
  await gifPage.fill('input[name="password"]', "admin123");
  await sleep(500);
  await gifPage.click('button[type="submit"]');
  try { await gifPage.waitForURL("**/dashboard", { timeout: 15000 }); } catch {}
  await sleep(3000);
  await gifPage.close();
  let vid = gifPage.video();
  if (vid) { const vp = await vid.path(); fs.renameSync(vp, path.join(VIDEO_DIR, "login-flow.webm")); console.log("  ✅ login-flow.webm"); }
  await gifCtx.close();

  // GIF 2: Module navigation tour
  gifCtx = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
    recordVideo: { dir: VIDEO_DIR, size: { width: 1440, height: 900 } },
  });
  gifPage = await gifCtx.newPage();
  await login(gifPage);
  await sleep(1000);
  const tourPages = ["/fi/accounts", "/mm/materials", "/sd/sales-orders", "/pp/production-orders", "/hr/employees", "/wm", "/qm", "/co"];
  for (const p of tourPages) {
    await gifPage.goto(`${BASE_URL}${p}`);
    await gifPage.waitForLoadState("networkidle");
    await sleep(2000);
  }
  await gifPage.close();
  vid = gifPage.video();
  if (vid) { const vp = await vid.path(); fs.renameSync(vp, path.join(VIDEO_DIR, "module-tour.webm")); console.log("  ✅ module-tour.webm"); }
  await gifCtx.close();

  // GIF 3: Create journal entry
  gifCtx = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
    recordVideo: { dir: VIDEO_DIR, size: { width: 1440, height: 900 } },
  });
  gifPage = await gifCtx.newPage();
  await login(gifPage);
  await gifPage.goto(`${BASE_URL}/fi/journal`);
  await gifPage.waitForLoadState("networkidle");
  await sleep(2000);
  const newBtn = gifPage.locator('a[href="/fi/journal/new"], button:has-text("新增"), button:has-text("New"), button:has-text("建立")');
  if (await newBtn.count() > 0) {
    await newBtn.first().click();
    await gifPage.waitForLoadState("networkidle");
    await sleep(3000);
  }
  await gifPage.close();
  vid = gifPage.video();
  if (vid) { const vp = await vid.path(); fs.renameSync(vp, path.join(VIDEO_DIR, "create-journal.webm")); console.log("  ✅ create-journal.webm"); }
  await gifCtx.close();

  // GIF 4: Sales orders detail
  gifCtx = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
    recordVideo: { dir: VIDEO_DIR, size: { width: 1440, height: 900 } },
  });
  gifPage = await gifCtx.newPage();
  await login(gifPage);
  await gifPage.goto(`${BASE_URL}/sd/sales-orders`);
  await gifPage.waitForLoadState("networkidle");
  await sleep(2000);
  const soLink = gifPage.locator("table tbody tr a, table tbody tr").first();
  if (await soLink.count() > 0) {
    await soLink.click();
    await gifPage.waitForLoadState("networkidle");
    await sleep(3000);
  }
  await gifPage.close();
  vid = gifPage.video();
  if (vid) { const vp = await vid.path(); fs.renameSync(vp, path.join(VIDEO_DIR, "sales-order-detail.webm")); console.log("  ✅ sales-order-detail.webm"); }
  await gifCtx.close();

  // GIF 5: Purchase order workflow
  gifCtx = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    locale: "zh-TW",
    recordVideo: { dir: VIDEO_DIR, size: { width: 1440, height: 900 } },
  });
  gifPage = await gifCtx.newPage();
  await login(gifPage);
  await gifPage.goto(`${BASE_URL}/mm/purchase-orders`);
  await gifPage.waitForLoadState("networkidle");
  await sleep(2000);
  const poLink = gifPage.locator("table tbody tr a, table tbody tr").first();
  if (await poLink.count() > 0) {
    await poLink.click();
    await gifPage.waitForLoadState("networkidle");
    await sleep(3000);
  }
  await gifPage.close();
  vid = gifPage.video();
  if (vid) { const vp = await vid.path(); fs.renameSync(vp, path.join(VIDEO_DIR, "purchase-order-flow.webm")); console.log("  ✅ purchase-order-flow.webm"); }
  await gifCtx.close();

  await browser.close();
  console.log("\n✨ All done!");
  console.log(`📁 Screenshots: ${SCREENSHOT_DIR}`);
  console.log(`📁 Videos: ${VIDEO_DIR}`);
}

main().catch(console.error);
