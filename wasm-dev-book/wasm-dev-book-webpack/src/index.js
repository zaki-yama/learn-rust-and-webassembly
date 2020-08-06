import('./wasm_dev_book_webpack').then(module => {
  const { add } = module
  console.log(add(1, 2));
});
