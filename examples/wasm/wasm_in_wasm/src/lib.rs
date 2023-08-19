#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
//! A rust wrapper for [WASM3](https://github.com/wasm3/wasm3).

extern crate alloc;

pub mod error;

mod environment;
pub use self::environment::Environment;
mod function;
pub use self::function::{CallContext, Function, RawCall};
mod macros;
pub use self::macros::*;
mod module;
pub use self::module::{Module, ParsedModule};
mod runtime;
pub use self::runtime::Runtime;
mod ty;
pub use self::ty::{WasmArg, WasmArgs, WasmType};
mod utils;
pub use ffi as wasm3_sys;

#[no_mangle]
pub extern "C" fn exec_wasm(a: u64, b: u64) -> u64 {
    let env = Environment::new().expect("Unable to create environment");
    // let rt = env
    //     .create_runtime(1024 * 60)
    //     .expect("Unable to create runtime");
    // let module = Module::parse(&env, &include_bytes!("../wasm_add/wasm_add.wasm")[..])
    //     .expect("Unable to parse module");
    // let module = rt.load_module(module).expect("Unable to load module");
    let i = 1;
    i
    // let func = module
    //     .find_function::<(i64, i64), i64>("add")
    //     .expect("Unable to find function");
}
