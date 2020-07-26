const js = import("./node_modules/@zaki-yama/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});
