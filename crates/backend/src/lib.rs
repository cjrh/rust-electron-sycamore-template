pub mod core;

use neon::prelude::*;

/// Returns a greeting from the Rust backend (Neon wrapper)
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(core::hello()))
}

/// Adds two numbers together (Neon wrapper)
fn add(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let b = cx.argument::<JsNumber>(1)?.value(&mut cx);
    Ok(cx.number(core::add(a, b)))
}

/// Returns system information (Neon wrapper)
fn get_system_info(mut cx: FunctionContext) -> JsResult<JsObject> {
    let info = core::get_system_info();
    let obj = cx.empty_object();

    let platform = cx.string(&info.platform);
    obj.set(&mut cx, "platform", platform)?;

    let arch = cx.string(&info.arch);
    obj.set(&mut cx, "arch", arch)?;

    let family = cx.string(&info.family);
    obj.set(&mut cx, "family", family)?;

    Ok(obj)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("add", add)?;
    cx.export_function("getSystemInfo", get_system_info)?;
    Ok(())
}
