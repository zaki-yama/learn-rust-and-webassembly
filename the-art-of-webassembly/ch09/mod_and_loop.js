const fs = require("fs");
const bytes = fs.readFileSync(__dirname + "/mod_and_loop.wasm");

(async () => {
  const obj = await WebAssembly.instantiate(new Uint8Array(bytes));
  let mod_loop = obj.instance.exports.mod_loop;
  let and_loop = obj.instance.exports.and_loop;

  let start_time = Date.now();
  mod_loop();
  console.log(`mod: ${Date.now() - start_time}`);

  start_time = Date.now();
  and_loop();
  console.log(`and: ${Date.now() - start_time}`);

  start_time = Date.now();
  for (let i = 0 | 0; i < 100_000_000; i++) {
    Math.floor(i % 1000);
  }
  console.log(`js mod: ${Date.now() - start_time}`);
})();
