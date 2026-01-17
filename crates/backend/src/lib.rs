use neon::prelude::*;

/// Returns a greeting from the Rust backend
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hello from Rust backend via Neon!"))
}

/// Adds two numbers together - demonstrates passing arguments
fn add(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let b = cx.argument::<JsNumber>(1)?.value(&mut cx);
    Ok(cx.number(a + b))
}

/// Returns system information - demonstrates returning objects
fn get_system_info(mut cx: FunctionContext) -> JsResult<JsObject> {
    let obj = cx.empty_object();

    let platform = cx.string(std::env::consts::OS);
    obj.set(&mut cx, "platform", platform)?;

    let arch = cx.string(std::env::consts::ARCH);
    obj.set(&mut cx, "arch", arch)?;

    let family = cx.string(std::env::consts::FAMILY);
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
