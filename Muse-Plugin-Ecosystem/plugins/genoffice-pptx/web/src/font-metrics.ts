import * as opentype from 'opentype.js'
import { Buffer } from 'buffer'
import { OpentypeMetrics, HeuristicMetrics, type FontMetricsProvider, type RunStyle } from '@genoffice/pptx-render'
import carlitoRegular from '../../vendor/genoffice/packages/ui/src/fonts/Carlito-Regular.ttf'
import carlitoBold from '../../vendor/genoffice/packages/ui/src/fonts/Carlito-Bold.ttf'
import carlitoItalic from '../../vendor/genoffice/packages/ui/src/fonts/Carlito-Italic.ttf'
import carlitoBoldItalic from '../../vendor/genoffice/packages/ui/src/fonts/Carlito-BoldItalic.ttf'
import arimoRegular from '@fontsource/arimo/files/arimo-latin-400-normal.woff'
import arimoBold from '@fontsource/arimo/files/arimo-latin-700-normal.woff'
import arimoItalic from '@fontsource/arimo/files/arimo-latin-400-italic.woff'
import arimoBoldItalic from '@fontsource/arimo/files/arimo-latin-700-italic.woff'

// Original GenOffice OpentypeMetrics + bundled, redistributable fonts. Keeping
// measurement and drawing on the same face prevents a platform's fallback font
// from changing line wraps. Carlito's Win metrics follow upstream fonts.ts.
const parsed=new Map<string,opentype.Font>()
const sources={carlito:[carlitoRegular,carlitoBold,carlitoItalic,carlitoBoldItalic],arimo:[arimoRegular,arimoBold,arimoItalic,arimoBoldItalic]}
const loads: Promise<FontFace>[]=[]
for(const [family,urls] of Object.entries(sources)) urls.forEach((url,i)=>{
 const bytes=Buffer.from(url.slice(url.indexOf(',')+1),'base64')
 const font=opentype.parse(bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength))
 if(family==='carlito') {font.ascender=font.tables.os2.usWinAscent;font.descender=-Math.abs(font.tables.os2.usWinDescent);(font as any).lineGap=0}
 else (font as any).lineGap=font.tables.hhea.lineGap||0
 parsed.set(`${family}:${i}`,font)
 const face=new FontFace(family==='carlito'?'Carlito GO':'Arimo GO',`url(${url})`,{weight:i%2?'700':'400',style:i>=2?'italic':'normal'})
 document.fonts.add(face);loads.push(face.load())
})
function family(style:RunStyle){
 const name=style.fontFamily.toLowerCase()
 if(['carlito','carlito go','calibri'].includes(name))return 'carlito'
 if(['arial','arimo','arimo go'].includes(name))return 'arimo'
 return null
}
const inner=new OpentypeMetrics(style=>{const f=family(style);return f?parsed.get(`${f}:${(style.bold?1:0)+(style.italic?2:0)}`):undefined},new HeuristicMetrics())
export const fontMetrics:FontMetricsProvider={
 metrics:style=>inner.metrics(style),measure:(text,style)=>inner.measure(text,style),
 displayFamily:style=>family(style)==='carlito'?'Carlito GO':family(style)==='arimo'?'Arimo GO':style.fontFamily,
}
export const fontsReady=()=>Promise.all(loads)
