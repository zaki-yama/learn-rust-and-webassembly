export const date_now = Date.now;

import('./wasm_dev_book_webpack').then(module => {
  const { add, get_timestamp } = module
  console.log(add(1, 2));
  console.log(get_timestamp());
});
