# Idle Hands itch.io Publishing Guide

This guide records the working release process for Idle Hands and the problems
encountered during its first itch.io publication. It is both a runbook for the
next release and a postmortem for diagnosing similar Rust/Macroquad games.

## Current release model

Idle Hands publishes two different products to the same itch.io project:

| Product | itch channel | Contents | Price/access |
| --- | --- | --- | --- |
| Browser demo | `html5-demo` | WebGL build with 30 playable games | Free, played in the page |
| Windows game | `windows` | Unrestricted 60-game Windows build | Pay what you want |

The project target is `webhatchery/idle-hands`. This is intentionally stored in
`itch.json`; Butler credentials are stored outside the repository. A Butler
login from another itch account works when that account has administrator
rights to the WebHatchery project.

Never commit Butler API keys, session credentials, secret page URLs, or account
cookies.

## Files that control publishing

- `itch.json` records the public project target, channel names, and version.
- `publish-itch.ps1` builds and stages the 30-game demo, invokes the shared
  RustGames Butler publisher, and restores the normal full WebGL artifacts.
- `itch-index.html` is the canvas-only page shipped inside the itch HTML5 ZIP.
- `Cargo.toml` declares the `demo` feature.
- `src/game_descriptor.rs` decides which 30 games are available in demo builds.
- `.cargo/config.toml` keeps analytics explicitly disabled until production
  details are ready.
- `src/main.rs` prepares the Rajdhani font atlas safely before the first WebGL
  UI frame.
- `tests/webgl-smoke.spec.mjs` verifies loading, touch input, sound activation,
  persistence, recovery, resizing, and browser errors for both the WebHatchery
  page and the canvas-only itch page.

Generated directories under `dist/` and `target-demo/` are artifacts, not
source files.

## One-time itch.io project settings

Use these values on the itch.io edit page:

### Project and pricing

- Kind of project: **HTML**
- Pricing: **$0 or donate**
- Browser demo upload: **This file will be played in the browser**
- Windows upload: mark as **Executable** and **Windows**
- Visibility: **Public** when the release is approved

The browser upload and Windows upload must remain separate. Do not mark the
Windows archive as the browser-playable file.

### Embed options

These settings match the useful behavior of Second Story:

- Run mode: **Embed in page**
- Size mode: **Manually set size**
- Width: **1280 px**
- Height: **720 px**
- Mobile friendly: **Enabled**
- Automatically start on page load: **Disabled**
- Fullscreen button: **Enabled**
- Scrollbars: **Disabled**
- SharedArrayBuffer support: **Disabled**
- Orientation: **Default**

With this configuration, clicking **Run game** replaces the launch panel with
an iframe while the itch.io header, downloads, description, and comments remain
on the page. Fullscreen happens only if the player presses itch's fullscreen
button.

Do not use **Click to launch in fullscreen**. It replaces the itch page and was
the cause of the initial full-page transition.

## Repeatable publishing procedure

Run all commands from the Idle Hands project directory.

### 1. Validate the normal release

```powershell
.\publish.ps1
```

This is the required project validation path. It builds and packages both the
normal Windows and WebGL editions and deploys the local preview. The normal
Windows artifact is the unrestricted 60-game build.

### 2. Preview the itch payload without uploading

```powershell
.\publish-itch.ps1 -Channel all -DryRun
```

The dry run should report:

- target `webhatchery/idle-hands`;
- browser channel `html5-demo`;
- Windows channel `windows`;
- a local, self-contained HTML5 package;
- `index.html` at the package root;
- no missing runtime or asset references.

For a browser-only check:

```powershell
.\publish-itch.ps1 -Channel html5 -DryRun
```

### 3. Run the packaged browser contract

The test server expects a directory containing an `idle_hands` child directory.
Copy the generated `dist/itch-webgl` package there, set
`IDLE_HANDS_WEB_ROOT` to the parent, and run:

```powershell
$smokeRoot = Join-Path $PWD ("dist\itch-smoke-" + (Get-Date -Format "yyyyMMddHHmmss"))
$smokeGame = Join-Path $smokeRoot "idle_hands"
New-Item -ItemType Directory -Path $smokeGame -Force | Out-Null
Copy-Item -Path "dist\itch-webgl\*" -Destination $smokeGame -Recurse -Force
$env:IDLE_HANDS_WEB_ROOT = $smokeRoot
npm ci
npm run test:webgl
```

The test must pass without page errors, console errors, failed local requests,
or stale WebGL texture warnings.

### 4. Upload

Upload both products:

```powershell
.\publish-itch.ps1 -Channel all
```

Or upload them independently:

```powershell
.\publish-itch.ps1 -Channel html5
.\publish-itch.ps1 -Channel windows
```

The HTML5 command always rebuilds the `demo` feature unless
`-SkipDemoBuild` is supplied. Only use `-SkipDemoBuild` when deliberately
reusing a demo WASM artifact that has already been verified.

### 5. Wait for itch processing

```powershell
.\publish-itch.ps1 -Channel html5 -Status
.\publish-itch.ps1 -Channel windows -Status
```

A successful Butler push can still be processing. Do not validate the public
page until the newest build has a check mark. Record the build number so browser
logs can be tied to the correct upload rather than a cached older build.

### 6. Verify the public page

Open <https://webhatchery.itch.io/idle-hands> in a fresh tab and check:

1. The page says **Published**, not Draft.
2. A **Run game** button is present before launch.
3. Clicking it does not navigate away or replace the itch page.
4. The iframe contains only the game canvas and loading state.
5. There is no second WebHatchery title, About section, controls list, footer,
   donation widget, or bug-report widget inside the iframe.
6. The game reaches the cabinet and accepts touch/click input.
7. The browser console has no panic, `RuntimeError: unreachable`, deleted
   texture, or failed-asset errors from the newest itch build URL.
8. The download section offers the Windows ZIP as name-your-own-price content.

## How the demo/full split works

The ordinary `publish.ps1` build is always the full game. The itch wrapper then:

1. builds WebGL with `--features demo` into `target-demo`;
2. copies the normal WebGL package structure into `dist/demo/webgl`;
3. replaces only the WASM module with the demo WASM;
4. replaces the generated WebHatchery page with `itch-index.html`;
5. temporarily presents that directory to the shared itch publisher;
6. restores the normal full `dist/webgl` directory in a `finally` block.

Keeping the demo in a separate Cargo target directory prevents the restricted
WASM module from being mistaken for the full Windows or WebHatchery build.

The demo feature keeps five games from each of six categories, for 30 playable
games total. Locked drawers remain visible and lead players toward the full
edition. Before uploading, confirm the demo cabinet displays **30 GAMES** and
the Windows build remains unrestricted.

## Canvas-only itch launcher

The normal `dist/webgl/index.html` is a complete WebHatchery product page. It is
correct for WebHatchery hosting, where it owns the title, About section,
controls, footer, fullscreen behavior, and surrounding layout. It is not the
right document to nest inside an itch product page, because itch already owns
all of that presentation.

`itch-index.html` therefore contains only:

- a temporary loading message;
- the Macroquad canvas;
- required runtime bridges;
- focus, touch, context-menu, and resize handling;
- the `load("idle_hands.wasm")` call.

Landscape displays keep a 16:9 canvas so input coordinates match the game's
1280×720 logical layout. Portrait displays fill the viewport so the game's
touch-first portrait layout can activate. The itch page supplies the optional
fullscreen button; the inner launcher does not add another one.

## Analytics state

Analytics are compile-time opt-in:

```toml
[env]
IDLE_HANDS_ANALYTICS_ENABLED = "false"
```

The dormant preview endpoint and write key may remain configured. Changing
those values does not enable collection. Do not set the enabled value to
`"true"` until the production endpoint, write key, privacy copy, and collection
approval are all ready.

## Problems encountered and their fixes

### Butler credentials belonged to another account

**Symptom:** The project lived under WebHatchery while the saved Butler login
belonged to the main Kalaith account.

**Cause:** Account ownership and upload authorization looked mismatched.

**Fix:** No credential duplication was needed. Butler accepted the existing
login because that account has administrator rights to the target project.
Only the non-secret target name belongs in `itch.json`.

### The HTML5 upload appeared attached but not playable

**Symptom:** itch listed the browser archive as a file, but did not offer a
working browser game.

**Cause:** An HTML upload must be attached to an HTML project and explicitly
marked **This file will be played in the browser**.

**Fix:** Set the project kind to HTML and mark the `html5-demo` upload as the
browser file. Keep the Windows archive as a separate executable upload.

### Clicking Run game replaced the entire itch page

**Symptom:** The itch header, downloads, and comments disappeared on launch.

**Cause:** The embed mode was **Click to launch in fullscreen** (`maximized`).

**Fix:** Change it to **Embed in page**, manually sized to 1280×720. Leave
automatic start off and expose itch's optional fullscreen button.

### The iframe repeated the WebHatchery About page

**Symptom:** The launched game showed another title, controls bar, About This
Game section, details, and footer inside itch.

**Cause:** The shared publisher copied the normal WebHatchery-generated
`dist/webgl/index.html`. Rewriting asset paths made it self-contained but did
not make it an itch-specific game launcher.

**Fix:** Add `itch-index.html` and have the project wrapper place it into the
demo package before Butler staging. The normal WebHatchery page remains
unchanged for WebHatchery deployment.

### The first WebGL demo rendered a black canvas

**Symptom:** The iframe and canvas existed, but the game was black. Browser logs
showed a panic in Macroquad's font atlas and `RuntimeError: unreachable`.

**Cause:** startup prewarming populated the full Latin glyph set at every font
size from 8 through 52 plus large display sizes. The WebGL atlas repeatedly
expanded until its image dimensions overflowed.

**Fix:** Keep Rajdhani, but cache a bounded representative character set at a
small set of useful sizes before real UI drawing begins. Present that off-screen
warm-up frame once, then start the game loop.

### Lazy font growth produced deleted-texture errors

**Symptom:** After removing excessive prewarming, the game rendered, but browser
tests logged repeated `glBindTexture called with an already deleted texture`
errors when opening new screens.

**Cause:** Macroquad can resize its font atlas while a frame is being batched.
Resizing replaces the GPU texture while earlier draw calls still reference the
old texture ID.

**Fix:** Grow the Rajdhani atlas deliberately during the bounded off-screen
startup warm-up. This creates enough capacity before any real UI draw calls are
queued.

Switching to Macroquad's default font was tested only as a diagnostic. It was
rejected because changing the established typography is not an acceptable
publishing fix.

### The minimal loading overlay would not disappear

**Symptom:** the WASM started, but the loading message remained visible in the
canvas-only package.

**Cause:** the launcher's `display: grid` rule overrode the browser's default
styling for the `hidden` attribute.

**Fix:** add an explicit `#loading[hidden] { display: none; }` rule and retain a
browser assertion that the overlay becomes hidden.

### Resize handling overflowed the JavaScript call stack

**Symptom:** the canvas-only page logged repeated `Maximum call stack size
exceeded` errors.

**Cause:** a window `resize` listener dispatched another window `resize` event,
recursively calling itself.

**Fix:** let `ResizeObserver` request Macroquad synchronization and do not attach
the redispatch function as a handler for the same event it emits.

### Touch targets were offset in non-16:9 desktop windows

**Symptom:** automated taps reached the canvas but missed visible controls.

**Cause:** stretching the canvas to an arbitrary landscape viewport introduced
letterboxing inside the game's virtual UI while the browser test mapped input
across the full canvas bounds.

**Fix:** constrain landscape canvas CSS to 16:9 and center it. Continue filling
the entire viewport in portrait, where Idle Hands intentionally uses its
portrait layout.

### The page remained Draft after the files were uploaded

**Symptom:** administrators could use the page and secret URL, but the public
could not discover or open it.

**Cause:** Butler publishes files and channels; it does not change the itch
project's visibility setting.

**Fix:** after final approval, select **Public — Anyone can view the page** and
save the itch edit form.

## Verification evidence from the first release

The completed release was verified with:

- the parameterless `publish.ps1` Windows/WebGL packaging path;
- Butler dry runs before live upload;
- warnings-as-errors Clippy;
- 898 Rust tests during the initial release setup;
- the Playwright shipping-browser contract against the packaged demo;
- a clean public-page launch after itch processing;
- confirmation that the itch header remained visible after launch;
- confirmation that the iframe contained one 1280×720 game canvas and no
  duplicate About section;
- browser logs tied to the newest build URL with no errors.

The canvas-only inline release first shipped as itch build `1917136`. Build
numbers are historical evidence, not configuration; always verify the newest
checked build reported by Butler.

## Fast troubleshooting map

| Symptom | First check |
| --- | --- |
| Upload downloads instead of playing | Project kind and browser-playable upload checkbox |
| Launch replaces itch | Embed mode is maximized instead of inline |
| Duplicate About/controls/footer | Packaged `index.html` is not `itch-index.html` |
| Black canvas | Browser console for WASM panic and newest build URL |
| Deleted texture warnings | Font atlas is growing during a real UI frame |
| Loading text covers game | `#loading[hidden]` CSS rule |
| Stack overflow on resize | Resize handler dispatches its own event |
| Touch misses buttons | Canvas aspect ratio and virtual-UI letterboxing |
| Old behavior after upload | Butler build still processing or browser/itch cache |
| Admin can view but public cannot | Visibility is still Draft or Restricted |
| Browser has all 60 games | Demo feature was not used for the HTML5 channel |
| Windows has only 30 games | Demo artifact contaminated the normal release path |

## Recommended policy for future WebHatchery itch releases

1. Treat an itch HTML5 ZIP as a game embed, not a duplicate storefront.
2. Keep demo and paid/full artifacts in separate build directories and channels.
3. Make privacy-sensitive integrations compile-time opt-in.
4. Test the exact staged itch package, not merely the normal WebHatchery build.
5. Verify itch's page settings separately because Butler cannot configure them.
6. Use a fresh browser tab and match console URLs to the newest processed build.
7. Preserve the game's typography and presentation when fixing platform issues.
8. Record the final channel names, embed settings, validation commands, and
   public-page checks in the project repository.
