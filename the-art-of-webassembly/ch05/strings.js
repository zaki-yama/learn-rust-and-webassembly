const fs = require("fs");
const bytes = fs.readFileSync(__dirname + "/strings.wasm");

const memory = new WebAssembly.Memory({ initial: 1 }); // linear memory

const importObject = {
  env: {
    buffer: memory,
    str_pos_len: (str_pos, str_len) => {
      const bytes = new Uint8Array(memory.buffer, str_pos, str_len);
      const log_string = new TextDecoder("utf8").decode(bytes);
      console.log(log_string);
    },
  },
};

(async () => {
  const obj = await WebAssembly.instantiate(
    new Uint8Array(bytes),
    importObject
  );
  const main = obj.instance.exports.main;

  main();
})();
