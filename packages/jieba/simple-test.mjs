import { Jieba } from '@node-rs/jieba'
const customDict = ['哪行 50', '干一行 51', '行一行 52', '行行 53']

const dictBuffer = Buffer.from(customDict.join('\n'), 'utf-8')
const jieba = Jieba.withDict(dictBuffer)

const text = '人要是行干一行行一行，一行行行行行，行行行干哪行都行'
const output = jieba.cut(text, false)
console.log('分词结果⤵️\n', output.join('/'))
