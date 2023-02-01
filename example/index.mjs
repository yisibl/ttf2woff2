import { promises as fs } from 'fs'
import path, { join } from 'path'
import { fileURLToPath } from 'url'
import { performance } from 'perf_hooks'
import * as fontkit from 'fontkit'

import { convertTTFToWOFF2, convertWOFF2ToTTF } from '../index.js'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.resolve(path.dirname(__filename))

async function toWOFF2() {
    const fontName = 'RobotoFlex-VF.ttf'
    // const fontName = 'fa-brands-400-v5.15.4.ttf'
    const ttfFont = await fs.readFile(join(__dirname, '/', fontName))
    const t = performance.now()
    const woff2Font = convertTTFToWOFF2(ttfFont)
    console.info('✨ TTF To WOFF2 Done in', performance.now() - t, 'ms')
    console.info('TTF font length  : ', ttfFont.length)
    console.info('WOFF2 font length: ', woff2Font.length)
    console.info('\n')

    await fs.writeFile(join(__filename, '..', fontName.replace(/\.ttf$/, '-2.woff2')), woff2Font)
}

async function toTTF() {
  const fontName = 'fa-brands-400-v6.2.woff2'
  // const fontName = 'fa-brands-400-v5.15.4.woff2'
  const woff2Font = await fs.readFile(join(__dirname, '/', fontName))

  const t = performance.now()
  const ttfFont = convertWOFF2ToTTF(woff2Font)
  console.info('✨ WOFF2 To TTF Done in', performance.now() - t, 'ms')
  console.info('WOFF2 font length: ', woff2Font.length)
  console.info('TTF font length  : ', ttfFont.length)
  console.info('\n')

  await fs.writeFile(join(__filename, '..', fontName.replace(/\.woff2$/, '.ttf')), ttfFont)
}

toWOFF2()
toTTF()

// readWOFF2()
function readWOFF2() {
  const fontName = 'fa-regular-400-bak.woff2'
  const fontPath = join(__dirname, '/', fontName)
  const font = fontkit.openSync(fontPath)

  console.log('font', font)
}
