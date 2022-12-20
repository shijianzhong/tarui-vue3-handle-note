/*
 * @Date: 2022-11-27 11:45:31
 * @LastEditors: shijianzhong shijianzhong
 * @LastEditTime: 2022-12-03 15:54:45
 * @FilePath: /vue-project/src-tauri/src/models.rs
 */
pub mod matter_item {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize)]
    #[warn(dead_code)]
    pub struct Item {
        pub item_id: String,
        pub item_name: String,
        pub item_type: String, //类型
        pub item_type_name: String,
        pub item_state: u8,
        pub item_desc: String,
        pub item_priority: String,
        pub item_start: String,
        pub item_end: String,
        pub item_desc_type: String //内容类型，1为markdown 2为画板
    }
    
    #[derive(Debug, Deserialize, Serialize)]
    #[warn(dead_code)]
    pub struct  ItemType{
        pub item_type:String,
        pub item_type_name:String,
        pub item_type_desc:String
    }
}
