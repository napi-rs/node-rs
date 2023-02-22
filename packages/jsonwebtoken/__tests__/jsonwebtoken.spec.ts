import { promises as fs } from 'node:fs'
import { join } from 'node:path'

import test from 'ava'
import { decode as nodeJwtDecode } from 'jsonwebtoken'

import { Algorithm, sign, signSync, verifySync, verify } from '../index.js'

const getUtcTimestamp = () => Math.floor(new Date().getTime() / 1000)
const oneDayInSeconds = 86400

test('signSync and sign (async) should produce the same result', async (t) => {
  const claims = {
    data: {
      id: 'f81d4fae-7dec-11d0-a765-00a0c91e6bf6',
      pr: 33,
      isM: true,
      set: ['KL', 'TV', 'JI'],
      nest: { id: 'poly' },
    },
    exp: getUtcTimestamp() + oneDayInSeconds,
  }
  const secretKey = 'secret'
  const resSync = signSync(claims, secretKey)
  const resAsync = await sign(claims, secretKey)

  t.is(resSync, resAsync)
  t.truthy(nodeJwtDecode(resAsync))
})

test('verify should return the decoded claims', async (t) => {
  const data = {
    id: 'f81d4fae-7dec-11d0-a765-00a0c91e6bf6',
    pr: 33,
    isM: true,
    set: ['KL', 'TV', 'JI'],
    nest: { id: 'poly' },
  }
  const claims = { data, exp: getUtcTimestamp() + oneDayInSeconds }
  const secretKey = 'secret'
  const headers = { algorithm: Algorithm.HS384 }

  const token = await sign(claims, secretKey, headers)

  const validation = { algorithms: [Algorithm.HS384] }
  const decodedToken = await verify(token, secretKey, validation)

  t.like(decodedToken, claims)
})

test('verify sync should return the decoded claims', async (t) => {
  const data = {
    id: 'f81d4fae-7dec-11d0-a765-00a0c91e6bf6',
    pr: 33,
    isM: true,
    set: ['KL', 'TV', 'JI'],
    nest: { id: 'poly' },
  }
  const claims = { data, exp: getUtcTimestamp() + oneDayInSeconds }
  const publicKeyPem =
    '-----BEGIN PUBLIC KEY-----MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAzq7L/V03tpy3QTYOP51CT0fY2Sp5spejcja9brkEZoLYFcvLSeNnsXtPg/Sr7PwbykiXoY++xo7+6o2VfPnbiEFV8fNap+4tWDmxeZfPifmCEA58BFncnK8z5luxR+syeRuI/9IUHllGxsKoQAbFECZoNCR+I5H/ynqhm9rvk89iNsh5EGxknOq2GmMaKRZ3nHZtVuwUj3BlwgsmP28ZAofMN/xM8bugHS1nNNHmRh6Ubg0Od3r2FH0+3df86ZzJ013M/LG1189aGNPXDOH4guBh7TPficw9nUnhIghiEFrxhAvIOQjClbhFud931T+UqD5BsF/ZarJ1VkaUa3UjxwIDAQAB-----END PUBLIC KEY-----'
  const privateKeyPem =
    '-----BEGIN RSA PRIVATE KEY-----MIIEowIBAAKCAQEAzq7L/V03tpy3QTYOP51CT0fY2Sp5spejcja9brkEZoLYFcvLSeNnsXtPg/Sr7PwbykiXoY++xo7+6o2VfPnbiEFV8fNap+4tWDmxeZfPifmCEA58BFncnK8z5luxR+syeRuI/9IUHllGxsKoQAbFECZoNCR+I5H/ynqhm9rvk89iNsh5EGxknOq2GmMaKRZ3nHZtVuwUj3BlwgsmP28ZAofMN/xM8bugHS1nNNHmRh6Ubg0Od3r2FH0+3df86ZzJ013M/LG1189aGNPXDOH4guBh7TPficw9nUnhIghiEFrxhAvIOQjClbhFud931T+UqD5BsF/ZarJ1VkaUa3UjxwIDAQABAoIBAQDGYRB7B9ZJ+PIMLY5PkOnsntGM4DAfM102a0Q32m5W1pABm7JsIVGOEQWpalb7CKDD8BlagVZjzyzuhSdO5aPJjKyppyMEvJ/ZZsbqJsSVcl9cegqfQoF2AtSV7ryigyXXCI7evQ2Cc75zWLOVgOn1LmgmZECOc7xI5JvptKLwAwrIuLE4wnuLgSdxxVZ8uwJHW7+hTCQ8x0cSID1POy3q39kEEdqi+yNOrVFZV7DGJ6T5gYWDe53fWpks++tr7D6Wbq1mRdX5T62IdG/G4q9/vA2tSR+5hZWMxMqZ+GUBmIH2zPU16yc4hfwne/C5WQkRUaPBIl/u5swFLHwxNIqBAoGBAPsCOB5T7/oO9Y/LyD6SCDLiKpKQhwPPZJ76/Nu9yNXM2sLINGDq6RUXmaflifoKSRxFqApBHXqcP8NRzrYT+eY5Q0/m2Nvt4MvoMRoNDx2FVnQY8yo4AdSpQl2fNhMdXc1R2Wc3EJdWZd+2J9xGBTbLZ5nUem9zdVdZr0YbMrwpAoGBANLK7txwi/YSYfHo+S0KZqqO32CAN8m0s6Clnz1SomZY4TX1nQQyfbzT06AG/7vtVf5roc4t1JrX08Qelu7VBOCH2Y2jEYyX1M6e7sJbl+Z5LYqOQkiAW+GBF3gvn/IvQ1Irjzd8MF/5wfyafaeE5mxoAtDOGW/BfcwORIoAOt5vAoGAHHjx+K64x/qubDNHcaGLAIqbHaj7R7lcxpPd3uc2QtpL7lBbcKr06YmVym/FKPHFvUlBeHhOabwTl4pOEmVNsYnJUuTysG/ZUgfymevlTQn09pJl8uILgx34AzquHZj1LPcd3BFo9mG8iJXXC6t9p+uGwvJRORc1tkTcFu264ZECgYB9sygXakH8PmAL6vrUQhSQ9tv75tndvZU0Yi+AWQug7rV2AP5eJ2HVvZfAIQxVW6VhL3vwwGG86KFOnVMyHvNmlXxFOw3XAh+UCzCj1AzUEkT3D/g01d50rg95yySdPlPt5y3jT3plcUGdyd7Oi7EAylGLhKukegTzLzrt9E8mnwKBgBx+31YGB/sxdLXKN7CKvkB9+PUQ1ywDZshzuXfSL+lEcgls6MT/AjMP49eEu14698S4VHnhMu/671TwJXS6NpCTCGjrUJoKymuaBGYvgFRqcqjVtHzyz+YMkFQISvi/DurN5CN4C1Yiv7EDFQC+69fcOo4tP9S9EFya189IvJsJ-----END RSA PRIVATE KEY-----'
  const headers = { algorithm: Algorithm.RS256 }

  const token = await sign(claims, privateKeyPem, headers)

  const validation = { algorithms: [Algorithm.RS256] }
  const decodedToken = verifySync(token, publicKeyPem, validation)

  t.like(decodedToken, claims)
})

test('verify should throw in case the token is expired', async (t) => {
  const data = {
    id: 'f81d4fae-7dec-11d0-a765-00a0c91e6bf6',
    pr: 33,
    isM: true,
    set: ['KL', 'TV', 'JI'],
    nest: { id: 'poly' },
  }
  const claims = { data, iat: getUtcTimestamp() - oneDayInSeconds * 2, exp: getUtcTimestamp() - oneDayInSeconds }
  const secretKey = 'secret'
  const headers = { algorithm: Algorithm.HS256 }

  const token = await sign(claims, secretKey, headers)

  t.throws(() => verifySync(token, secretKey))
  await t.throwsAsync(() => verify(token, secretKey))
})

test('should allow to use a buffer as sign/verify key', async (t) => {
  const [publicKeyPem, privateKeyPem] = await Promise.all([
    fs.readFile(join(__dirname, 'public-key.pem')),
    fs.readFile(join(__dirname, 'private-key.pem')),
  ])

  const data = {
    id: 'f81d4fae-7dec-11d0-a765-00a0c91e6bf6',
    pr: 33,
    isM: true,
    set: ['KL', 'TV', 'JI'],
    nest: { id: 'poly' },
  }
  const claims = { data, exp: getUtcTimestamp() + oneDayInSeconds }
  const headers = { algorithm: Algorithm.PS256 }

  const token = await sign(claims, privateKeyPem.toString(), headers)

  const validation = { algorithms: [Algorithm.PS256] }
  const decodedToken = await verify(token, publicKeyPem.toString(), validation)

  t.like(decodedToken, claims)
})
