import { expect, test } from "@playwright/test";

const LOGICAL_WIDTH = 1280;
const LOGICAL_HEIGHT = 720;

async function tapGame(page, logicalX, logicalY) {
  const box = await page.locator("#glcanvas").boundingBox();
  expect(box).not.toBeNull();
  await page.touchscreen.tap(
    box.x + (logicalX / LOGICAL_WIDTH) * box.width,
    box.y + (logicalY / LOGICAL_HEIGHT) * box.height,
  );
  await page.waitForTimeout(400);
}

test("tower placement is inspectable, cancellable and explicitly committed", async ({ page }) => {
  await page.goto("/idle_hands/", { waitUntil: "domcontentloaded" });
  await page.waitForFunction(() => Boolean(window.wasm_exports), undefined, { timeout: 30_000 });
  await expect(page.locator("#loading")).toBeHidden();
  await page.waitForTimeout(500);

  await tapGame(page, 716, 466);
  await tapGame(page, 868, 145);
  await tapGame(page, 890, 626);
  const board = await page.locator("#glcanvas").screenshot({ animations: "disabled" });

  await tapGame(page, 1032, 130);
  await tapGame(page, 470, 245);
  const inspection = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  expect(inspection.equals(board)).toBe(false);

  await tapGame(page, 998, 616);
  const built = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  expect(built.equals(inspection)).toBe(false);

  await tapGame(page, 470, 245);
  const upgradeInspection = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  expect(upgradeInspection.equals(built)).toBe(false);
  await tapGame(page, 1142, 616);
  const cancelled = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  expect(cancelled.equals(built)).toBe(false);
});
