import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

import { Bench } from 'tinybench'
import nodejieba from 'nodejieba'

import { Jieba } from '../index.js'
import { dict } from '../dict.js'

const { load, cut, tag } = nodejieba

const __dirname = join(fileURLToPath(import.meta.url), '..')

const fixture = readFileSync(join(__dirname, 'weicheng.txt'), 'utf8')

const preface = `
重印前记《围城》一九四七年在上海初版，一九四八年再版，一九四九年三版，以后国内没有重印过。偶然碰见它的新版，那都是香港的“盗印”本。没有看到台湾的“盗印”，据说在那里它是禁书。美国哥伦比亚大学夏志清教授的英文著作里对它作了过高的评价，导致了一些西方语言的译本。日本京都大学荒井健教授很久以前就通知我他要翻译，近年来也陆续在刊物上发表了译文。现在，人民文学出版社建议重新排印，以便原著在国内较易找着，我感到意外和忻辛。
我写完《围城》，就对它不很满意。出版了我现在更不满意的一本文学批评以后，我抽空又长篇小说，命名《百合心》，也脱胎于法文成语（Iecoeurd“artichaut），中心人物是一个女角。大约已写成了两万字。一九四九年夏天，全家从上海迁居北京，手忙脚乱中，我把一叠看来像乱纸的草稿扔到不知哪里去了。兴致大扫，一直没有再鼓起来，倒也从此省心省事。年复一年，创作的冲动随年衰减，创作的能力逐渐消失——也许两者根本上是一回事，我们常把自己的写作冲动误认为自己的写作才能，自以为要写就意味着会写。相传幸运女神偏向着年轻小伙子，料想文艺女神也不会喜欢老头儿的；不用说有些例外，而有例外正因为有公例。我慢慢地从省心进而收心，不作再写小说的打算。事隔三十余年，我也记不清楚当时腹稿里的人物和情节。就是追忆清楚了，也还算不得数，因为开得出菜单并不等于摆得成酒席，要不然，谁都可以马上称为善做菜的名厨师又兼大请客的阔东道主了，秉承曹雪芹遗志而拟定”后四十回“提纲的学者们也就可以凑得成和的得上一个或半个高鹗了。剩下来的只是一个顽固的信念：假如《百合心》写得成，它会比《围城》好一点。事情没有做成的人老有这类根据不充分的信念；我们对采摘不到的葡萄，不但想像它酸，也很可能想像它是分外地甜。
这部书禄版时的校读很草率，留下不少字句和标点的脱误，就无意中为翻译者安置了拦路石和陷阱。我乘重印的机会，校看一遍，也顺手有节制地修必了一些字句。《序》里删去一节，这一节原是郑西谛先生要我添进去的。在去年美国出版的珍妮·凯利（JeanneKelly）女士和茅国权（NathanK.Mao）先生的英译本里，那一节已省去了。
一九八0年二月这本书第二次印刷，我又改正了几个错字。两次印刷中，江秉祥同志给了技术上和艺术上的帮助，特此志谢。
一九八一年二月我乘第三次印刷的机会，修订了一些文字。有两处多年朦混过去的讹误，是这本书的德译者莫妮克（MonikaMotsch）博士发觉的。
一九八二年十二月为了塞尔望——许来伯（SylvieServan-Schreiber）女士的法语译本，我去年在原书里又校正了几外错漏，也修改了几处词句。恰好这本书又要第次印刷，那些改正就可以安插了。苏联索洛金（V.Sorokin）先生去年提醒我，他的俄译本比原著第一次重印本早问世五个月，我也借此带便提一下。
`

const prefaceLength = preface.length

async function createBench(
  suitename: string,
  transform: (o: string[]) => string,
  napi: () => any[],
  jieba: () => any[],
) {
  const suite = new Bench({
    name: suitename,
  })
  console.assert(transform(napi()) === transform(jieba()))

  suite.add('@node-rs/jieba', napi).add('nodejieba', jieba)

  await suite.run()

  console.table(suite.table())
}

load()
const jieba = Jieba.withDict(dict)

await createBench(
  `Cut ${prefaceLength} words`,
  (output) => output.join(''),
  () => jieba.cut(preface),
  () => cut(preface),
)

await createBench(
  `Cut ${fixture.toString().length} words`,
  (output) => output.join(''),
  () => jieba.cut(fixture),
  () => cut(fixture),
)

await createBench(
  `Tag ${prefaceLength} words`,
  (output) => typeof output,
  () => jieba.tag(preface),
  () => tag(preface),
)

await createBench(
  `Tag ${fixture.toString().length} words`,
  (output) => typeof output,
  () => jieba.tag(fixture),
  () => tag(fixture),
)
