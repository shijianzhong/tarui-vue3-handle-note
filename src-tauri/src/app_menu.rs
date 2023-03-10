/*
 * @Date: 2022-11-26 22:34:27
 * @LastEditors: shijianzhong 994129509@qq.com
 * @LastEditTime: 2023-02-16 16:21:12
 * @FilePath: /vue-project/src-tauri/src/app_menu.rs
 */
// use tauri::api::dialog::message;
use tauri::utils::assets::EmbeddedAssets;
use tauri::{AboutMetadata, Context, CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

use crate::MAIN_WINDOW;
pub fn init(context: &Context<EmbeddedAssets>) -> Menu {
    // 应用名称
    let name = &context.package_info().name;
    // tauri::Menu::os_default(name)
    // 应用主菜单

    let mut about_meta_data = AboutMetadata::new();
    about_meta_data.comments =Some(String::from("sking"));
    about_meta_data.version =Some(String::from("v0.0.1"));
    println!("{:#?}",about_meta_data);
    // let app_menu = Submenu::new(
    //     "",
    //     // MenuItem::About 为原生菜单
    //     Menu::new().add_native_item(MenuItem::About(name.into(), about_meta_data)),
    // );
    let app_menu = Submenu::new(
        name,
        Menu::with_items([
            #[cfg(target_os = "macos")]
            MenuItem::About(name.into(), AboutMetadata::default()).into(),
            #[cfg(not(target_os = "macos"))]
            CustomMenuItem::new("about".to_string(), "About").into(),
            CustomMenuItem::new("check_update".to_string(), "Check for Updates").into(),
            MenuItem::Services.into(),
            MenuItem::Hide.into(),
            MenuItem::HideOthers.into(),
            MenuItem::ShowAll.into(),
            MenuItem::Separator.into(),
            MenuItem::Quit.into(),
        ])
    );
    // 文件菜单（自定义菜单）
    let file_menu = Submenu::new(
        "事项",
        Menu::new()
            .add_item(CustomMenuItem::new("new_item_type".to_string(), "新建类型"))
            .add_item(CustomMenuItem::new("new_item".to_string(), "新建事项")),
    );
    // 编辑菜单（自定义菜单）
    let edit_menu = Submenu::new(
        "能力",
        Menu::new()
            .add_item(CustomMenuItem::new("open_ai".to_string(), "智能问答"))
            .add_item(CustomMenuItem::new("note".to_string(), "笔记")),
    );

    Menu::new()
        .add_submenu(app_menu)
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
}
// 应用菜单处理事件
pub fn handler(event: WindowMenuEvent) {
    // 菜单所属的窗口
    // let win = Some(event.window());
    // 匹配菜单 id
    match event.menu_item_id() {
        "new_item_type" => {
            // debug 信息（终端输出）
            event_to_front("new_item_type");
        }
        "new_item" => {
            // 发送信息到菜单所属窗口（弹窗形式）
            // message(win, "Eidt File", "TODO");
            dbg!("edit file");
            event_to_front("new_item");
        }
        "open_ai" =>{
            dbg!("open_ai");
            event_to_front("open_ai");
        }
        "note" => {
            event_to_front("note");
        }
        "redo" => {
            dbg!("redo");
        }
        _ => {}
    }
}
#[derive(Clone, serde::Serialize)]
struct ToFrontMessage {
    message: String,
}
fn event_to_front(emit_type: &str) {
    unsafe {
        let window = MAIN_WINDOW.clone().unwrap();
        window
            .emit(
                "rust_event",
                ToFrontMessage {
                    message: emit_type.to_string(),
                },
            )
            .unwrap();
    }
}
