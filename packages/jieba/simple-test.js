const { cut } = require('./index')

const [first, second] = cut('武汉市长江大桥')

console.assert(first === '武汉市')
console.assert(second === '长江大桥')
