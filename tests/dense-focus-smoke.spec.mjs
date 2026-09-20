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
  await page.waitForTimeout(250);
}

test("dense Sudoku focus mode keeps neighboring selection and entry touchable", async ({ page }) => {
  await page.goto("/idle_hands/", { waitUntil: "domcontentloaded" });
  await page.waitForFunction(() => Boolean(window.wasm_exports), undefined, { timeout: 30_000 });
  await expect(page.locator("#loading")).toBeHidden();
  await page.waitForTimeout(500);

  await tapGame(page, 602, 605);
  await tapGame(page, 890, 626);
  const board = await page.locator("#glcanvas").screenshot({ animations: "disabled" });

  await tapGame(page, 940, 130);
  const focus = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  expect(focus.equals(board)).toBe(false);

  await tapGame(page, 314, 198);
  await tapGame(page, 548, 190);
  const entered = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  expect(entered.equals(focus)).toBe(false);

  await tapGame(page, 1090, 662);
  const returned = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  expect(returned.equals(focus)).toBe(false);
});

test("dense Nonogram focus mode pans without marking intermediate cells", async ({ page }) => {
  await page.goto("/idle_hands/", { waitUntil: "domcontentloaded" });
  await page.waitForFunction(() => Boolean(window.wasm_exports), undefined, { timeout: 30_000 });
  await expect(page.locator("#loading")).toBeHidden();
  await page.waitForTimeout(500);

  await tapGame(page, 1008, 605);
  await tapGame(page, 890, 626);
  const board = await page.locator("#glcanvas").screenshot({ animations: "disabled" });

  await tapGame(page, 940, 120);
  const focus = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  expect(focus.equals(board)).toBe(false);

  await tapGame(page, 358, 412);
  await tapGame(page, 470, 412);
  const panned = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  expect(panned.equals(focus)).toBe(false);

  await tapGame(page, 386, 270);
  const marked = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  expect(marked.equals(panned)).toBe(false);

  await tapGame(page, 1090, 662);
  const returned = await page.locator("#glcanvas").screenshot({ animations: "disabled" });
  expect(returned.equals(marked)).toBe(false);
});
