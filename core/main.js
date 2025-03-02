import("./pkg/qpace.js").then((wasm) => {
  // mod.initSync();
  // wasm.initSync();
  wasm.greet();
});
