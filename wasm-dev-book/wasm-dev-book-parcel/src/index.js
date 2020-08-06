import { add, rand } from './lib.rs';

const toUint32 = num => num >>> 0;

console.log(add(1, 2));
console.log(toUint32(rand()));
