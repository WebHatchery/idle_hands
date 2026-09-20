import { expect, test } from "@playwright/test";

async function waitForGame(page) {
  await page.waitForFunction(() => Boolean(window.wasm_exports), undefined, {
    timeout: 30_000,
  });
  await expect(page.locator("#loading")).toBeHidden();
  await page.waitForTimeout(400);
}

async function tapGame(page, x, y, width = 1280, height = 720) {
  const box = await page.locator("#glcanvas").boundingBox();
  expect(box).not.toBeNull();
  await page.touchscreen.tap(
    box.x + (x / width) * box.width,
    box.y + (y / height) * box.height,
  );
  await page.waitForTimeout(300);
}

test("cabinet keeps resume, browse, utility, favorite and drawer routes touchable", async ({ page }) => {
  await page.goto("/idle_hands/", { waitUntil: "domcontentloaded" });
  await waitForGame(page);

  const freshHome = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  await tapGame(page, 420, 150);
  await tapGame(page, 890, 626);
  expect((await page.locator("#glcanvas").screenshot({ animations: "disabled" })).equals(freshHome)).toBe(false);
  await tapGame(page, 80, 45);

  await tapGame(page, 400, 300);
  const category = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  await tapGame(page, 100, 238);
  await tapGame(page, 1105, 670);
  expect((await page.locator("#glcanvas").screenshot({ animations: "disabled" })).equals(category)).toBe(false);

  await tapGame(page, 470, 145);
  const favorited = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  await tapGame(page, 420, 145);
  expect((await page.locator("#glcanvas").screenshot({ animations: "disabled" })).equals(favorited)).toBe(false);
  await tapGame(page, 1015, 525);
  await tapGame(page, 300, 50);

});
