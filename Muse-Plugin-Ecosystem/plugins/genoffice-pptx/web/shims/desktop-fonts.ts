// Browser binding for the original Desktop session's font provider. No file
// system scanning crosses the plugin's sandbox; redistributable faces are local.
import { fontMetrics } from '../src/font-metrics'
export const createSystemFontMetrics = () => fontMetrics
export const resetFontRegistry = () => {}
