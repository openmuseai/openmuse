import { serveStdio } from '@modelcontextprotocol/server/stdio'
import { buildServer } from './server_factory.js'

serveStdio(buildServer)
