pub mod config;
pub mod core;

use config::AppConfig;
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

/// Loads configuration from disk and returns it as JSON (Neon wrapper)
fn load_config(mut cx: FunctionContext) -> JsResult<JsString> {
    match AppConfig::load() {
        Ok(config) => {
            let json = serde_json::to_string(&config).unwrap_or_else(|_| "{}".to_string());
            Ok(cx.string(json))
        }
        Err(_) => {
            let default = AppConfig::default();
            let json = serde_json::to_string(&default).unwrap_or_else(|_| "{}".to_string());
            Ok(cx.string(json))
        }
    }
}

/// Saves configuration to disk (Neon wrapper)
fn save_config(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let json = cx.argument::<JsString>(0)?.value(&mut cx);

    let result = serde_json::from_str::<AppConfig>(&json)
        .map_err(|_| ())
        .and_then(|config| config.save().map_err(|_| ()));

    Ok(cx.boolean(result.is_ok()))
}

/// Returns the config file path (Neon wrapper)
fn get_config_path(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = AppConfig::config_path()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    Ok(cx.string(path))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("add", add)?;
    cx.export_function("getSystemInfo", get_system_info)?;
    cx.export_function("loadConfig", load_config)?;
    cx.export_function("saveConfig", save_config)?;
    cx.export_function("getConfigPath", get_config_path)?;
    Ok(())
}
