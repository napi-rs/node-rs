const { Worker, isMainThread, parentPort } = require('worker_threads')

function run() {
  return new Promise((resolve, reject) => {
    const worker = new Worker(__filename)
    worker.on('message', (message) => {
      resolve(message)
    })
    worker.on('error', reject)
    worker.on('exit', (code) => {
      if (code !== 0) {
        reject(new Error(`Worker stopped with exit code ${code}`))
      }
    })
  })
}

if (isMainThread) {
  async function main() {
    while (true) {
      const result = await Promise.all(Array.from({ length: 12 }).map(() => run()))
      console.info(result)
    }
  }
  main().catch((e) => {
    console.error(e)
  })
} else {
  const { hash: argon2 } = require('@node-rs/argon2')
  const { hash: bcrypt } = require('@node-rs/bcrypt')
  const { crc32 } = require('@node-rs/crc32')
  Promise.all([argon2('password'), bcrypt('password')]).then(([a, b]) => {
    parentPort.postMessage([crc32(a), crc32(b)])
  })
}
