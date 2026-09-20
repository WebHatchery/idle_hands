import { expect, test } from "@playwright/test";

const LOGICAL_WIDTH = 1280;
const LOGICAL_HEIGHT = 720;

async function waitForGame(page) {
  await page.waitForFunction(() => Boolean(window.wasm_exports), undefined, {
    timeout: 30_000,
  });
  await expect(page.locator("#loading")).toBeHidden();
  await expect(page.locator("#glcanvas")).toBeVisible();
  await page.waitForFunction(() => {
    const canvas = document.querySelector("#glcanvas");
    return canvas && canvas.width > 0 && canvas.height > 0;
  });
  await page.waitForTimeout(350);
}

async function tapGame(page, logicalX, logicalY) {
  const box = await page.locator("#glcanvas").boundingBox();
  expect(box).not.toBeNull();
  await page.touchscreen.tap(
    box.x + (logicalX / LOGICAL_WIDTH) * box.width,
    box.y + (logicalY / LOGICAL_HEIGHT) * box.height,
  );
  await page.waitForTimeout(250);
}

async function canvasImage(page) {
  return page.locator("#glcanvas").screenshot({ animations: "disabled" });
}

test("the deployed WebGL game survives its shipping-browser contract", async ({ page }) => {
  const failures = [];
  let phase = "load";
  page.on("pageerror", (error) => failures.push(`${phase} page: ${error.message}`));
  page.on("console", (message) => {
    if (message.type() === "error") failures.push(`${phase} console: ${message.text()}`);
  });
  page.on("requestfailed", (request) => {
    if (request.url().startsWith("http://127.0.0.1:4173")) {
      failures.push(`${phase} request: ${request.url()} (${request.failure()?.errorText})`);
    }
  });

  await page.route("https://storage.ko-fi.com/**", async (route) => {
    await route.fulfill({
      contentType: "text/javascript",
      body: "window.kofiWidgetOverlay={draw:function(){}};",
    });
  });
  await page.addInitScript(() => {
    window.__idleHandsSmoke = {
      audioContexts: [],
      audioResumeCalls: 0,
      audioStarts: 0,
      touchStarts: 0,
    };
    window.addEventListener(
      "touchstart",
      () => {
        window.__idleHandsSmoke.touchStarts += 1;
      },
      { capture: true },
    );
    const NativeAudioContext = window.AudioContext || window.webkitAudioContext;
    const InstrumentedAudioContext = new Proxy(NativeAudioContext, {
      construct(Target, args) {
        const context = Reflect.construct(Target, args);
        window.__idleHandsSmoke.audioContexts.push(context);
        const resume = context.resume.bind(context);
        context.resume = (...resumeArgs) => {
          window.__idleHandsSmoke.audioResumeCalls += 1;
          return resume(...resumeArgs);
        };
        const createBufferSource = context.createBufferSource.bind(context);
        context.createBufferSource = (...sourceArgs) => {
          const source = createBufferSource(...sourceArgs);
          const start = source.start.bind(source);
          source.start = (...startArgs) => {
            window.__idleHandsSmoke.audioStarts += 1;
            return start(...startArgs);
          };
          return source;
        };
        return context;
      },
    });
    window.AudioContext = InstrumentedAudioContext;
    window.webkitAudioContext = InstrumentedAudioContext;
  });

  await page.goto("/idle_hands/", { waitUntil: "domcontentloaded" });
  await waitForGame(page);
  const desktopCanvas = await page.locator("#glcanvas").evaluate((canvas) => ({
    width: canvas.width,
    height: canvas.height,
  }));
  const originalCabinet = await canvasImage(page);

  // Touch the visible settings control and verify that the game's procedural
  // sound actually starts from a user activation.
  phase = "touch-and-audio";
  await tapGame(page, 1216, 52);
  expect(await page.evaluate(() => window.__idleHandsSmoke.touchStarts)).toBeGreaterThan(0);
  expect((await canvasImage(page)).equals(originalCabinet)).toBe(false);
  await expect.poll(async () => page.evaluate(() => window.__idleHandsSmoke.audioResumeCalls)).toBeGreaterThan(0);
  await expect.poll(async () => page.evaluate(() =>
    window.__idleHandsSmoke.audioContexts.some((context) => context.state === "running"),
  )).toBe(true);
  expect(await page.evaluate(() => window.__idleHandsSmoke.audioStarts)).toBeGreaterThan(0);

  // Persist a conspicuous accessibility setting, save it explicitly, then
  // reload the full WASM page and verify both the data and rendered restoration.
  phase = "persistence";
  await tapGame(page, 405, 492);
  await tapGame(page, 485, 584);
  await expect.poll(async () => page.evaluate(() => localStorage.length)).toBeGreaterThan(0);
  const profile = await page.evaluate(() => {
    const raw = localStorage.getItem("idle_hands_save_autosave_profile");
    return raw ? JSON.parse(raw) : null;
  });
  expect(profile?.slot?.version).toBe("1.0.0");
  expect(profile?.data?.version).toBe("1.0.0");
  expect(profile?.data?.high_contrast).toBe(true);

  await page.reload({ waitUntil: "domcontentloaded" });
  phase = "reload";
  await waitForGame(page);
  const restoredCabinet = await canvasImage(page);
  expect(restoredCabinet.equals(originalCabinet)).toBe(false);
  expect(
    await page.evaluate(() => JSON.parse(localStorage.getItem("idle_hands_save_autosave_profile")).data.high_contrast),
  ).toBe(true);

  // Enter 2048 entirely by touch, dismiss its exact visible tutorial control,
  // and prove the visible NEW GAME / CANCEL recovery path restores the board.
  phase = "recovery";
  await tapGame(page, 440, 150);
  await tapGame(page, 890, 626);
  const board = await canvasImage(page);
  await tapGame(page, 1020, 444);
  const confirmation = await canvasImage(page);
  expect(confirmation.equals(board)).toBe(false);
  await tapGame(page, 530, 384);
  const recovered = await canvasImage(page);
  expect(recovered.equals(board)).toBe(true);

  // Resize to a phone-shaped viewport. The WebHatchery storefront supplies its
  // own fullscreen controls; the itch package intentionally ships only canvas.
  phase = "resize-and-fullscreen";
  await page.setViewportSize({ width: 430, height: 844 });
  await page.waitForTimeout(500);
  const mobileCanvas = await page.locator("#glcanvas").evaluate((canvas) => ({
    width: canvas.width,
    height: canvas.height,
  }));
  const mobileBox = await page.locator("#glcanvas").boundingBox();
  expect(mobileCanvas).not.toEqual(desktopCanvas);
  expect(mobileBox.width).toBeLessThanOrEqual(430);
  expect(mobileBox.height).toBeGreaterThan(0);

  const fullscreenButton = page.getByRole("button", { name: "Play full screen" });
  if (await fullscreenButton.count()) {
    await fullscreenButton.tap();
    await expect.poll(async () => page.evaluate(() =>
      Boolean(document.fullscreenElement || document.webkitFullscreenElement
        || document.body.classList.contains("game-playing")),
    )).toBe(true);
    await expect(page.getByRole("button", { name: "Leave full screen" })).toBeVisible();
    await page.getByRole("button", { name: "Leave full screen" }).tap();
    await expect.poll(async () => page.evaluate(() =>
      Boolean(document.fullscreenElement || document.webkitFullscreenElement
        || document.body.classList.contains("game-playing")),
    )).toBe(false);
  } else {
    expect(mobileBox.width).toBe(430);
    expect(mobileBox.height).toBe(844);
  }

  await page.waitForTimeout(250);
  expect(failures).toEqual([]);
});
