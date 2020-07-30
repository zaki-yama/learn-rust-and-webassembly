async function run() {
  let response = await fetch("../main.wasm");
  let bytes = await response.arrayBuffer();
  let module = await WebAssembly.instantiate(bytes, {
    env: {
      wasm_log(start, len) {
        // 開始位置と長さから、テキストを抽出する
        const utf8dec = new TextDecoder("utf-8");
        let memory = new Uint8Array(module.instance.exports.memory.buffer);
        let text = utf8dec.decode(memory.subarray(start, start + len));
        document.getElementById("container").innerHTML += text+"<br>";

      },
    },
  });

  // Tur "Ferris" into bytes
  const utf8enc = new TextEncoder("utf-8");
  let text = "Ferris";
  let text_bytes = utf8enc.encode(text);

  // Allocate enough space for the text
  let len = text_bytes.length;
  let start = module.instance.exports.wasm_malloc(len);

  // Put the text in WebAssembly program's memory
  let memory = new Uint8Array(module.instance.exports.memory.buffer);
  memory.set(text_bytes, start);

  module.instance.exports.main(start, len);
}

run();
