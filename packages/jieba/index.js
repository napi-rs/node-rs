const { loadBinding } = require('@node-rs/helper')

const native = loadBinding(require, __dirname, 'jieba', '@node-rs/jieba')

module.exports = {
  ...native,
  cut: function cut(sentence, hmm = false) {
    const input = Buffer.isBuffer(sentence) ? sentence : Buffer.from(sentence)
    return native.cut(input, hmm).split(',')
  },
  cutAll: function cutAll(sentence) {
    return native.cutAll(Buffer.isBuffer(sentence) ? sentence : Buffer.from(sentence)).split(',')
  },
  cutForSearch: function cutForSearch(sentence, hmm = false) {
    const input = Buffer.isBuffer(sentence) ? sentence : Buffer.from(sentence)
    return native.cutForSearch(input, hmm)
  },
  tag: function tag(sentence, hmm = false) {
    const input = Buffer.isBuffer(sentence) ? sentence : Buffer.from(sentence)
    const output = native.tag(input, hmm)
    return output.split(',').map((tagged) => {
      const [tag, word] = tagged.split('|')
      return { tag, word }
    })
  },

  extract: function extract(sentence, topn, allowedPos = []) {
    const input = Buffer.isBuffer(sentence) ? sentence : Buffer.from(sentence)
    return native.extract(input, topn, allowedPos.join(','))
  },
}
