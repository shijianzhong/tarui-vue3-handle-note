/*
 * @Date: 2022-11-01 14:27:55
 * @LastEditors: shijianzhong 994129509@qq.com
 * @LastEditTime: 2023-02-08 13:36:42
 * @FilePath: /vue-project/src-tauri/src/main.rs
 */
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
extern crate dirs;
mod app_menu;
mod db_controller;
use db_controller::Dao;
mod models;
use models::matter_item::{Item,ItemType};
use tauri::{ Manager, Window};
// static mut CURRENT_APP: &App;
static mut MAIN_WINDOW: Option<Window> = None;
fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .setup(move |app| {
      unsafe {
        MAIN_WINDOW = app.get_window("main");
      }
      Ok(())
    })
    .menu(app_menu::init(&context))
    .on_menu_event(app_menu::handler)
    .invoke_handler(tauri::generate_handler![greet, get_items, add_item,add_item_type,vitural_delete_item,
      get_item_types,delete_item,update_item,get_trash_items])
    .run(context)
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello,{}", name)
}
#[tauri::command]
fn get_items() -> String {
  let dao = Dao::create();
  if dao.check_table_existed_else_created("todo_items") {}
  let tmp = dao.list_items().unwrap();
  let data = serde_json::to_string(&tmp).unwrap();
  data
}
#[tauri::command]
fn get_trash_items() -> String {
  let dao = Dao::create();
  if dao.check_table_existed_else_created("todo_items") {}
  let tmp = dao.list_trash_items().unwrap();
  let data = serde_json::to_string(&tmp).unwrap();
  data
}
#[tauri::command]
fn get_item_types() -> String {
  let dao = Dao::create();
  if dao.check_table_existed_else_created("todo_items") {}
  let tmp = dao.list_item_type().unwrap();
  let data = serde_json::to_string(&tmp).unwrap();
  data
}
#[tauri::command]
fn vitural_delete_item(item_id:String) -> bool {
  // println!("收到假删除参数{}",item_id);
  let dao = Dao::create();
  dao.vitural_delete_item(item_id) //假删除
}
#[tauri::command]
fn delete_item(item_id:String) -> bool {
  // println!("收到真的删除参数{}",item_id);
  let dao = Dao::create();
  dao.delete_item(item_id)  //真删除
}
#[tauri::command]
fn update_item(item:String) -> bool {
  // println!("更新item对象：{}",item);
  let dao = Dao::create();
  let parsed:Item = serde_json::from_str(item.as_str()).unwrap();
  dao.update_item(parsed) //
}
#[tauri::command]
fn add_item(form: &str) -> String {
  // println!("this is  fe's value :{}", form);
  let parsed: Item = serde_json::from_str(form).unwrap();
  // println!("{:#?}", parsed);
  let dao = Dao::create();
  if dao.check_table_existed_else_created("todo_items") {
    // println!("nothing");
  }
  let r = dao.add_item(parsed);
  if let Some(true) = r {
    "1".to_string()
  } else {
    "0".to_string()
  }
}
#[tauri::command]
fn add_item_type(form: &str) -> bool {
  // println!("this is  fe's value :{}", form);
  let parsed: ItemType = serde_json::from_str(form).unwrap();
  // print!("{:#?}", parsed);
  let dao = Dao::create();
  if dao.check_table_existed_else_created("todo_items") {
    // println!("nothing");
  }
  let r = dao.add_item_type(parsed);
  if let Some(true) = r {
    true
  } else {
    false
  }
}