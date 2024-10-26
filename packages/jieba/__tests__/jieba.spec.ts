import test from 'ava'

import { Jieba, TfIdf } from '../index.js'
import { dict, idf } from '../dict.js'

const sentence = '我是拖拉机学院手扶拖拉机专业的。不用多久，我就会升职加薪，走上人生巅峰。'

const jieba = Jieba.withDict(dict)
const tfIdf = TfIdf.withDict(idf)

test('cut result should be equal to nodejieba', (t) => {
  t.snapshot(jieba.cut(sentence))
})

test('tag result shoule be equal to nodejieba', (t) => {
  t.snapshot(jieba.tag(sentence))
})

test('extract should be equal to nodejieba', (t) => {
  const sentence =
    '今天纽约的天气真好啊，京华大酒店的张尧经理吃了一只北京烤鸭。后天纽约的天气不好，昨天纽约的天气也不好，北京烤鸭真好吃'
  const topn = 3
  t.snapshot(
    tfIdf.extractKeywords(jieba, sentence, topn).map((t) => ({
      keyword: t.keyword,
      weight: typeof t.weight,
    })),
  )
})

test('should be able to load custom TFID dict', (t) => {
  const userdict = Buffer.from('专业 20.19')
  const tfIdf = TfIdf.withDict(userdict)
  const fixture = '我是拖拉机学院手扶拖拉机专业的。不用多久，我就会升职加薪，当上CEO，走上人生巅峰。'
  t.snapshot(tfIdf.extractKeywords(jieba, fixture, 3))
})

test('should be able to load custom dict', (t) => {
  const userdict = Buffer.from('出了 10000')
  const jieba = Jieba.withDict(userdict)
  const fixture = '我们中出了一个叛徒'
  t.notThrows(() => jieba.cut(fixture))
})
