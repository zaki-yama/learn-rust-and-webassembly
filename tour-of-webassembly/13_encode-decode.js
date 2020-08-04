let bytes = new ArrayBuffer(8);
const data = new Uint8Array(bytes);
data[0] = 72;  // H
data[1] = 105; // i
data[2] = 33;  // !
let str_len = 3;
const utf8dec = new TextDecoder("utf-8");
// subarrayはその範囲をインデックス指定する
let text = utf8dec.decode(data.subarray(0,str_len));
console.log(text) // Hi!
const utf8enc = new TextEncoder("utf-8");
let text_bytes = utf8enc.encode(text);
console.log(text_bytes)
// Uint8Array(3) [72, 105, 33]
