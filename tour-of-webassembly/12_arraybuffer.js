// 8バイトの領域を初期化
let bytes = new ArrayBuffer(8);

// 8ビット符号なし整数としてビューを取得
let u8_bytes = new Uint8Array(bytes);
// ArrayBufferを操作
u8_bytes[0] = 16; // 00010000
console.log(u8_bytes[0]);
u8_bytes[1] = 1;  // 00000001
console.log(u8_bytes[1]);
console.log(u8_bytes);

// 32ビット符号あり整数のビューとして改めて変換
let i32_bytes = new Int32Array(u8_bytes.buffer);
console.log(i32_bytes[0]);
///272 or 00010000000000010000000000000000
