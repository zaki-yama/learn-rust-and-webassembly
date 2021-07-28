const fs = require("fs");
const loader = require("@assemblyscript/loader");

const importObject = {
  as_hello: {
    console_log: (str_index) => {
      console.log(module.exports.__getString(str_index));
    },
  },
};

(async () => {
  module = await loader.instantiate(
    fs.readFileSync(__dirname + "/as_concat.wasm")
  );
  // __newString, __getString functions require
  // compile with --exportRuntime flag
  const first_str_index = module.exports.__newString("first string");
  const second_str_index = module.exports.__newString("second string");
  const cat_str_index = module.exports.cat(first_str_index, second_str_index);
  const cat_string = module.exports.__getString(cat_str_index);
  console.log(cat_string);
})();
