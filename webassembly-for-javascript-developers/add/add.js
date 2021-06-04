const fs = require("fs");

const run = async () => {
  try {
    const bytecode = fs.readFileSync("add/add.wasm");
    const wasm = await WebAssembly.instantiate(bytecode);
    console.log(wasm.instance.exports.addInt32(1, 2));
    // TypeError: wasm function signature contains illegal type
    // console.log(wasm.instance.exports.addInt64(1, 2));
  } catch (e) {
    console.error(e);
  }
};

run().then();
