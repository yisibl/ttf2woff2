import { readFile } from 'fs/promises'
import { join } from 'path'
import { fileURLToPath } from 'url'

import * as fontkit from 'fontkit'
import test from 'ava'

import { convertTTFToWOFF2, convertTTFToWOFF2Async, convertWOFF2ToTTF, convertWOFF2ToTTFAsync } from '../index.js'

const fixture = await readFile(join(fileURLToPath(import.meta.url), '..', './iconsfont.ttf'))
const fontawesome = await readFile(join(fileURLToPath(import.meta.url), '..', './fa-brands-400-v6.2.woff2'))

test('should be able to convert ttf to woff2', (t) => {
  t.notThrows(() => convertTTFToWOFF2(fixture))
})

test('should be able to convert ttf to woff2 async', async (t) => {
  await t.notThrowsAsync(() => convertTTFToWOFF2Async(fixture))
})

test('convert woff2 to ttf', (t) => {
  t.notThrows(() => convertWOFF2ToTTF(fontawesome))
})

test('convert woff2 back to ttf', (t) => {
  const woff2 = convertTTFToWOFF2(fixture)
  t.notThrows(() => convertWOFF2ToTTF(woff2))
})

test('convert woff2 back to ttf async', async (t) => {
  const woff2 = await convertTTFToWOFF2Async(fixture)
  await t.notThrowsAsync(() => convertWOFF2ToTTFAsync(woff2))
})
