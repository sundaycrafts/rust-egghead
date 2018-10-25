import("./app").then(module => {
  console.log('loaded...')
});

import("my-wasm-lib").then(module => {
  module.greet("World")
})
