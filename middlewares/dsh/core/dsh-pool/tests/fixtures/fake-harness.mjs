import { createServer } from "node:http";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const port = Number(process.env.PORT ?? "0");
const server = createServer((req, res) => {
  const url = req.url ?? "/";
  if (url.includes("capabilities")) {
    res.writeHead(200, { "content-type": "application/json" });
    res.end(JSON.stringify({ ok: true, protocol: "muse.parent-bridge/v1" }));
    return;
  }
  res.writeHead(200, { "content-type": "text/plain" });
  res.end("ok");
});
process.stdout.write("dsh web: http://127.0.0.1/?token=launch-fixture\n");
server.listen(port, "127.0.0.1");
void dirname;
void join;
void fileURLToPath;
