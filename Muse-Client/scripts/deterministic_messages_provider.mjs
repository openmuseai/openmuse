import { createServer } from "node:http";

const port = Number(process.env.PORT ?? "13250");
const responseDelayMs = Number(process.env.DETERMINISTIC_RESPONSE_DELAY_MS ?? "0");

function messageText(content) {
  if (typeof content === "string") return content;
  if (!Array.isArray(content)) return "";
  return content
    .filter(block => block?.type === "text" && typeof block.text === "string")
    .map(block => block.text)
    .join("\n");
}

function frame(type, value) {
  return `event: ${type}\ndata: ${JSON.stringify({ type, ...value })}\n\n`;
}

const server = createServer((request, response) => {
  let body = "";
  request.setEncoding("utf8");
  request.on("data", chunk => { body += chunk; });
  request.on("end", () => {
    let marker = "OPENMUSE-DETERMINISTIC-RESPONSE";
    try {
      const payload = JSON.parse(body);
      const text = (Array.isArray(payload.messages) ? payload.messages : [])
        .filter(message => message?.role === "user")
        .map(message => messageText(message.content))
        .join("\n");
      marker = [...text.matchAll(/(?:CLOUD|DESKTOP)-[A-Z0-9-]+/gu)].at(-1)?.[0] ?? marker;
    } catch {
      // DSH validates request shape; this fixture keeps its response bounded.
    }

    const complete = () => {
      response.writeHead(200, {
        "content-type": "text/event-stream",
        "cache-control": "no-cache",
      });
      response.write(frame("message_start", {
        message: {
          id: "msg_openmuse_acceptance",
          type: "message",
          role: "assistant",
          content: [],
          model: "deepseek-chat",
          stop_reason: null,
          stop_sequence: null,
          usage: { input_tokens: 3, output_tokens: 0 },
        },
      }));
      response.write(frame("content_block_start", {
        index: 0,
        content_block: { type: "text", text: "" },
      }));
      response.write(frame("content_block_delta", {
        index: 0,
        delta: { type: "text_delta", text: marker },
      }));
      response.write(frame("content_block_stop", { index: 0 }));
      response.write(frame("message_delta", {
        delta: { stop_reason: "end_turn", stop_sequence: null },
        usage: { output_tokens: 1 },
      }));
      response.end(frame("message_stop", {}));
    };

    if (responseDelayMs > 0 && marker.startsWith("DESKTOP-RUNNING-")) {
      setTimeout(complete, responseDelayMs);
    } else {
      complete();
    }
  });
});

server.listen(port, "127.0.0.1", () => {
  process.stdout.write(`deterministic-messages-provider http://127.0.0.1:${port}\n`);
});
