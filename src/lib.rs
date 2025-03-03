use extism_pdk::*;
use serde::Serialize;

// use this function to run any command from host OS
// to run an executable from plugins data folder just run it with /data/<executable_name>
// returns commands stdout
#[host_fn]
extern "ExtismHost" {
    fn cli_run(command: String, args: Json<Vec<String>>) -> String;
}

#[derive(Serialize)]
struct PluginCommand {
    name: String,
    description: String,
    icon: String,
}

#[plugin_fn]
pub fn init() -> FnResult<Json<Vec<PluginCommand>>> {
    // this value comes from pluginArgs, user can set it from plugin settings
    // you can change or delete this from manifest.json
    let item_text = config::get("item text")
        .unwrap_or(Some("".to_string()))
        .unwrap_or("".to_string());

    // items to show in the plugin
    let items = vec![PluginCommand {
        name: format!("{}", item_text),
        description: "description of the item".to_string(),
        icon: "".to_string(),
    }];
    Ok(Json(items))
}

#[plugin_fn]
pub fn filter(query: String) -> FnResult<Json<Vec<PluginCommand>>> {
    // Add your filtering logic here, or return empty list to let puppet handle filter
    Ok(Json(vec![]))
}

#[plugin_fn]
pub fn on_select(selected: String) -> FnResult<()> {
    // Add your selection handling logic here
    log!(LogLevel::Info, "on_select :: {}", selected);
    Ok(())
}
