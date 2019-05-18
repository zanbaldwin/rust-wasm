import Application from './index';
const WasmModule = import('../build/pkg/wasm');

WasmModule.then(module => {
    const app = new Application(module);
    app.run();
}).catch(console.error);