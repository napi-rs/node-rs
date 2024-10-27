import { Bench } from 'tinybench'
import chalk from 'chalk'
import { crc32 as crc32Node } from 'crc'
import Sse4Crc32 from 'sse4_crc32'

import { crc32c, crc32 } from '../index.js'

const TEST_BUFFER = Buffer.from(`Lorem ipsum dolor sit amet, consectetur
adipiscing elit. Morbi mollis cursus metus vel tristique. Proin congue massa
massa, a malesuada dolor ullamcorper a. Nulla eget leo vel orci venenatis
placerat. Donec semper condimentum justo, vel sollicitudin dolor consequat id.
Nunc sed aliquet felis, eget congue nisi. Mauris eu justo suscipit, elementum
turpis ut, molestie tellus. Mauris ornare rutrum fringilla. Nulla dignissim
luctus pretium. Nullam nec eros hendrerit sapien pellentesque sollicitudin.
Integer eget ligula dui. Mauris nec cursus nibh. Nunc interdum elementum leo, eu
sagittis eros sodales nec. Duis dictum nulla sed tincidunt malesuada. Quisque in
vulputate sapien. Sed sit amet tellus a est porta rhoncus sed eu metus. Mauris
non pulvinar nisl, volutpat luctus enim. Suspendisse est nisi, sagittis at risus
quis, ultricies rhoncus sem. Donec ullamcorper purus eget sapien facilisis, eu
eleifend felis viverra. Suspendisse elit neque, semper aliquet neque sed,
egestas tempus leo. Duis condimentum turpis duis.`)

const initialCrc32 = crc32Node(TEST_BUFFER)
const initialCrc32c = Sse4Crc32.calculate(TEST_BUFFER)

console.assert(crc32(TEST_BUFFER) === initialCrc32)
console.assert(crc32c(TEST_BUFFER) === initialCrc32c)

const suite = new Bench()

await suite
  .add('@node/rs crc32c', () => {
    crc32c(TEST_BUFFER)
  })
  .add('sse4_crc32', () => {
    Sse4Crc32.calculate(TEST_BUFFER)
  })
  .warmup()

await suite.run()

console.info(chalk.green('crc32c without initial crc'))
console.table(suite.table())

const suite2 = new Bench()

await suite2
  .add('@node/rs crc32c', () => {
    crc32c(TEST_BUFFER, initialCrc32c)
  })
  .add('sse4_crc32', () => {
    Sse4Crc32.calculate(TEST_BUFFER, initialCrc32c)
  })
  .warmup()

await suite2.run()

console.info(chalk.green('crc32c with initial crc'))
console.table(suite2.table())

const suite3 = new Bench()

await suite3
  .add('@node/rs crc32', () => {
    crc32(TEST_BUFFER)
  })
  .add('Node crc', () => {
    crc32Node(TEST_BUFFER)
  })
  .warmup()

await suite3.run()

console.info(chalk.green('crc32 without initial crc'))
console.table(suite3.table())

const suite4 = new Bench()

await suite4
  .add('@node/rs crc32', () => {
    crc32(TEST_BUFFER, initialCrc32)
  })
  .add('Node crc32', () => {
    crc32Node(TEST_BUFFER, initialCrc32)
  })
  .warmup()

await suite4.run()

console.info(chalk.green('crc32 with initial crc'))
console.table(suite4.table())
