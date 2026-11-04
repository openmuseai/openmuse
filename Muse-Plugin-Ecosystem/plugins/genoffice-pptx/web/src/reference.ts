import { readFileSync, writeFileSync } from 'node:fs'
import { openPptx } from '@genoffice/pptx-engine'
import { buildAllRenderSlides } from '../../vendor/genoffice/apps/slides/src/main/session-state'
async function main(){const opened=await openPptx(readFileSync(process.argv[2]));writeFileSync(process.argv[3],JSON.stringify(buildAllRenderSlides(opened,1280)))}
void main()
