# Install pbjs 

```bash
pnpm install -g protobufjs-cli
```

# Generate javascript from protobuf files

```bash
# generate javascript
pbjs -t static-module -w es6 -o ./src/proto/messages.js ./src/proto/messages.proto

# generate typescript definitions
pbts -o ./src/proto/messages.d.ts ./src/proto/messages.js
```