async function run() {
  let response = await fetch("../main.wasm");
  let bytes = await response.arrayBuffer();
  let module = await WebAssembly.instantiate(bytes, {
    env: {
      wasm_log(start, len) {
        // 開始位置と長さから、テキストを抽出する
        const utf8dec = new TextDecoder("utf-8");
        let buffer = module.instance.exports.memory.buffer;
        let memory = new Uint8Array(buffer);
        let text = utf8dec.decode(memory.subarray(start, start + len));
        document.getElementById("container").innerHTML += text+"<br>";

      },
    },
  });
  module.instance.exports.main();
}

run();
