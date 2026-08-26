import { createReadStream, existsSync, statSync } from "node:fs";
import { createServer } from "node:http";
import path from "node:path";

const root = path.resolve(
  process.env.IDLE_HANDS_WEB_ROOT ?? path.join("dist", "browser-smoke", "games"),
);

if (!existsSync(path.join(root, "idle_hands", "index.html"))) {
  throw new Error(`Packaged WebGL deployment not found beneath ${root}`);
}

const mimeTypes = new Map([
  [".css", "text/css; charset=utf-8"],
  [".html", "text/html; charset=utf-8"],
  [".js", "text/javascript; charset=utf-8"],
  [".json", "application/json; charset=utf-8"],
  [".png", "image/png"],
  [".ttf", "font/ttf"],
  [".wasm", "application/wasm"],
  [".zip", "application/zip"],
]);

createServer((request, response) => {
  const requestPath = decodeURIComponent(new URL(request.url, "http://localhost").pathname);
  const relative = requestPath.replace(/^\/+/, "");
  let filePath = path.resolve(root, relative);
  if (!filePath.startsWith(`${root}${path.sep}`) && filePath !== root) {
    response.writeHead(403).end("Forbidden");
    return;
  }
  if (existsSync(filePath) && statSync(filePath).isDirectory()) {
    filePath = path.join(filePath, "index.html");
  }
  if (!existsSync(filePath) || !statSync(filePath).isFile()) {
    response.writeHead(404).end("Not found");
    return;
  }

  response.writeHead(200, {
    "Cache-Control": "no-store",
    "Content-Type": mimeTypes.get(path.extname(filePath).toLowerCase()) ?? "application/octet-stream",
  });
  createReadStream(filePath).pipe(response);
}).listen(4173, "127.0.0.1");
