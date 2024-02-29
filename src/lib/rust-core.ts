/// <reference path="../../static/wasm/rust_core.d.ts" />
type RustCoreWasm = typeof wasm_bindgen
class RustCore {
    get wasm() : RustCoreWasm{
        // @ts-ignore
        return wasm_bindgen;
    }
}

var rustCore = new RustCore();
export default rustCore;