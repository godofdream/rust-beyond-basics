

    (async () => {
        const fetchPromise = fetch('inc.wasm');
        const { instance } = await WebAssembly.instantiateStreaming(fetchPromise,{});
        const wasm = instance.exports;
        wasm.greet("Bernd")
    })();


