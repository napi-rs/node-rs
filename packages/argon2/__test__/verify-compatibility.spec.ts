import { readFileSync } from 'node:fs'
import test from 'ava'
import { verify, verifySync, type VerifyOptions } from '../index.js'

const corpus = JSON.parse(readFileSync(new URL('./historical-hashes.json', import.meta.url), 'utf8')) as {
  fixtures: { generatorVersion: string; password: string; secretHex: string | null; hash: string }[]
}

test('secret-only verification accepts frozen PHC hashes from published releases', async (t) => {
  for (const row of corpus.fixtures) {
    const options: VerifyOptions = row.secretHex === null ? {} : { secret: Buffer.from(row.secretHex, 'hex') }
    t.true(verifySync(row.hash, row.password, options), row.generatorVersion)
    t.true(await verify(row.hash, row.password, options), row.generatorVersion)
    t.false(await verify(row.hash, 'wrong', options), row.generatorVersion)
    if (row.secretHex !== null) {
      t.false(verifySync(row.hash, row.password))
      t.false(await verify(row.hash, row.password, { secret: Buffer.from('wrong') }))
    }
  }
})
