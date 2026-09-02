#!/usr/bin/env node
// Forwards 0.0.0.0:LISTEN_PORT -> 127.0.0.1:DSH_LOOPBACK_PORT.
// Official DSH refuses --host 0.0.0.0. Docker publish still needs a
// container-wide listener; host compose bind remains 127.0.0.1.
import net from "node:net";

const listenPort = Number(process.env.PORT || 3080);
const targetPort = Number(process.env.DSH_LOOPBACK_PORT || 13080);
const targetHost = "127.0.0.1";

const server = net.createServer((client) => {
  const upstream = net.connect({ host: targetHost, port: targetPort }, () => {
    client.pipe(upstream);
    upstream.pipe(client);
  });
  const fail = () => {
    client.destroy();
    upstream.destroy();
  };
  client.on("error", fail);
  upstream.on("error", fail);
});

server.on("error", (err) => {
  console.error("loopback-proxy:", err.message);
  process.exit(1);
});

server.listen(listenPort, "0.0.0.0");
