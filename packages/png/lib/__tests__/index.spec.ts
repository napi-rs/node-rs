import path from 'path'
import fs from 'fs'
import sharp from 'sharp'
import test from 'ava'
import { promisify } from 'util'

const readFileAsync = promisify(fs.readFile)

import { decode } from '../index'

test('should decode png', async (t) => {
  const pngPath = path.join(__dirname, 'fixtures', 'sigi.png')
  const pngData = await readFileAsync(pngPath)
  const pixels = await decode(pngData)
  console.log(pixels)
  const sharpPixels = await sharp(pngData)
    .raw()
    .toBuffer()
  console.log(sharpPixels)
  t.is(pixels.length, sharpPixels.length)
  // t.deepEqual(pixels, sharpPixels)
})
