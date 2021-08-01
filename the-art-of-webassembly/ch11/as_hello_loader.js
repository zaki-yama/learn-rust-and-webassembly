const loader = require("@assemblyscript/loader");

const fs = require("fs");

const importObject = {
  as_hello: {
    console_log: (str_index) => {
      console.log(module.exports.__getString(str_index));
    },
  },
};

(async () => {
  const wasm = fs.readFileSync(__dirname + "/as_hello.wasm");
  module = await loader.instantiate(wasm, importObject);
  module.exports.HelloWorld();
})();
