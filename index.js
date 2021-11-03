const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'the-super-tiny-compiler-in-rust' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `the-super-tiny-compiler-in-rust.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `the-super-tiny-compiler-in-rust-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'the-super-tiny-compiler-in-rust', 'the-super-tiny-compiler-in-rust')


module.exports.compiler = function compiler(input) {
  let tokens = module.exports.tokenizer(input);
  let ast    = module.exports.parser(tokens);
  let newAst = module.exports.transformer(ast);
  let output = module.exports.codeGenerator(newAst);

  // and simply return the output!
  return output;
}
