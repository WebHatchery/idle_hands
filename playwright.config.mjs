import { defineConfig } from "@playwright/test";
import path from "node:path";
import { fileURLToPath } from "node:url";

const projectRoot = path.dirname(fileURLToPath(import.meta.url));
const webRoot = process.env.IDLE_HANDS_WEB_ROOT
  ? path.resolve(process.env.IDLE_HANDS_WEB_ROOT)
  : path.resolve(projectRoot, "dist", "browser-smoke", "games");

export default defineConfig({
  testDir: path.resolve(projectRoot, "tests"),
  fullyParallel: false,
  retries: process.env.CI ? 1 : 0,
  reporter: process.env.CI ? "github" : "line",
  timeout: 60_000,
  use: {
    baseURL: "http://127.0.0.1:4173",
    browserName: "chromium",
    hasTouch: true,
    viewport: { width: 1365, height: 900 },
  },
  webServer: {
    command: "node scripts/serve_webgl_smoke.mjs",
    env: { IDLE_HANDS_WEB_ROOT: webRoot },
    reuseExistingServer: false,
    timeout: 15_000,
    url: "http://127.0.0.1:4173/idle_hands/",
  },
});
