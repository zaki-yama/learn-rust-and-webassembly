const fs = require("fs");

const run = async () => {
  try {
    const bytecode = fs.readFileSync("echo/echo.wasm");
    const imports = {
      env: {
        printNumber: (arg) => {
          console.log(arg);
        },
      },
    };
    const wasm = await WebAssembly.instantiate(bytecode, imports);
    console.log(WebAssembly.Module.imports(wasm.module));
    console.log(WebAssembly.Module.exports(wasm.module));
    wasm.instance.exports.echo(2021);
  } catch (e) {
    console.error(e);
  }
};

run().then();
