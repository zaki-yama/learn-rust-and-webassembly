const fs = require("fs");
const bytes = fs.readFileSync(__dirname + "/mod_and.wasm");

(async () => {
  const obj = await WebAssembly.instantiate(new Uint8Array(bytes));
  let mod = obj.instance.exports.mod;
  let and = obj.instance.exports.and;
  let start_time = Date.now();
  // The '| 0' syntax is a hint to the JavaScript engine to tell it
  // to use integers instead of floats, which can improve performance in
  // some circumstances
  for (let i = 0 | 0; i < 4_000_000; i++) {
    mod(i);
  }
  console.log(`mod: ${Date.now() - start_time}`);
  start_time = Date.now();

  for (let i = 0 | 0; i < 4_000_000; i++) {
    and(i);
  }
  console.log(`and: ${Date.now() - start_time}`);
  start_time = Date.now();

  for (let i = 0 | 0; i < 4_000_000; i++) {
    Math.floor(i % 1000);
  }
  console.log(`js mod: ${Date.now() - start_time}`);
})();
