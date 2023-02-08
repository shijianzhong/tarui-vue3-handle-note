/*
 * @Date: 2022-11-27 10:00:09
 * @LastEditors: shijianzhong 994129509@qq.com
 * @LastEditTime: 2023-02-08 13:39:18
 * @FilePath: /vue-project/src-tauri/src/db_controller.rs
 */

use crate::models::matter_item::ItemType;
use crate::Item;
use rusqlite::{params, Connection, Result};
use uuid::Uuid;
pub struct Dao {
    con: Connection,
}
impl Dao {
    pub fn create_table(&self) {
        let sql = "CREATE TABLE 'todo_items' (
            'item_id' text NOT NULL ON CONFLICT FAIL,
            'item_name' TEXT,
            'item_type' integer,
            'item_desc' TEXT,
            'item_state' integer,
            'item_priority' integer,
            'item_start' TEXT,
            'item_end' TEXT,
            'item_delete_flag' integer DEFAULT 0,
            'item_desc_type' text,
            CONSTRAINT 'item_type' FOREIGN KEY ('item_type') REFERENCES 'item_type' ('item_type') ON DELETE CASCADE ON UPDATE CASCADE );
            ";
        self.con.execute(sql, params![]).unwrap();
        let sql1: &str = "CREATE TABLE 'item_type' (
            'item_type' integer NOT NULL,
            'item_type_name' TEXT NOT NULL,
            'item_type_desc' TEXT,
            PRIMARY KEY ('item_type')
          );";
        self.con.execute(sql1, params![]).unwrap();
    }
    pub fn check_table_existed_else_created(&self, table_name: &str) -> bool {
        let sql = "select count(`name`) from 'sqlite_master' where `type` = 'table' and `name` = ?";
        let mut stmt = self.con.prepare(sql).unwrap();
        let rs = stmt.query_row(params![table_name], |row| {
            return row.get(0) as Result<i32>;
        });
        let count = rs.unwrap();
        // println!("长度：：：：{}", count);
        if count > 0 {
            return true;
        } else {
            self.create_table();
            return true;
        }
    }
    pub fn create() -> Dao {
        let db_file_path = dirs::data_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        let db_file_path = format!("{}/do.db", db_file_path.as_str());
        let con = Connection::open(&db_file_path).unwrap();
        Dao { con }
    }
    pub fn delete_item(&self, item_id: String) -> bool {
        let sql: String = format!("DELETE FROM `todo_items` where `item_id` = '{}';", item_id);
        // println!("{}", &sql);
        let size = self.con.execute(&sql, []);
        match size {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
    pub fn vitural_delete_item(&self, item_id: String) -> bool {
        let sql: String = format!(
            "update `todo_items` set `item_delete_flag`= 1,
            item_type =1024
            where `item_id` ='{}';",
            item_id
        );
        let size = self.con.execute(&sql, []);
        match size {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    pub fn update_item(&self, item: Item) -> bool {
        // println!("{:#?}", item);
        let sql: String = format!(
            "update `todo_items` set `item_name`= '{}',
            `item_type`={},`item_desc`='{}',`item_state`={},
            `item_priority`={},`item_start`='{}',`item_end`='{}',`item_desc_type`='{}'
            where `item_id` ='{}';",
            item.item_name,
            item.item_type,
            item.item_desc,
            item.item_state,
            item.item_priority,
            item.item_start,
            item.item_end,
            item.item_desc_type,
            item.item_id
        );
        // println!("sql语句::::{}", sql);
        let size = self.con.execute(&sql, []);
        // println!("{:#?}", size);
        match size {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    pub fn add_item(&self, item: Item) -> Option<bool> {
        let sql = format!(
            "INSERT INTO `todo_items` (item_id,item_name,
            item_type,item_desc,item_state,item_priority,item_start,item_end,item_desc_type
            ) VALUES('{}','{}',{},'{}',{},{},'{}','{}','{}');",
            Uuid::new_v4(),
            item.item_name.as_str(),
            item.item_type.as_str(),
            item.item_desc.as_str(),
            item.item_state,
            item.item_priority.as_str(),
            item.item_start.as_str(),
            item.item_end.as_str(),
            item.item_desc_type.as_str()
        );
        // println!("{}", sql);
        let size = self.con.execute(&sql, []);
        match size {
            Ok(v) => Some(v == 1),
            Err(e) => {
                eprintln!("错误：{}", e);
                return Some(false);
            }
        }
    }
    pub fn add_item_type(&self, item_type: ItemType) -> Option<bool> {
        let sql = format!(
            "INSERT INTO `item_type` VALUES(null,'{}','{}');",
            item_type.item_type_name.as_str(),
            item_type.item_type_desc.as_str()
        );
        // println!("add_item_type::::{}", sql);
        let size = self.con.execute(&sql, []);
        match size {
            Ok(v) => Some(v == 1),
            Err(e) => {
                eprintln!("错误：{}", e);
                return Some(false);
            }
        }
    }
    pub fn list_item_type(&self) -> Result<Vec<ItemType>> {
        let sql = "select t1.item_type,t1.item_type_name,t1.item_type_desc from item_type as t1";
        let mut stmt = self.con.prepare(sql).unwrap();
        let item_iterator = stmt
            .query_map([], |row| {
                Ok(ItemType {
                    item_type: row.get::<usize, i32>(0)?.to_string(),
                    item_type_name: row.get(1)?,
                    item_type_desc: row.get(2)?,
                })
            })
            .unwrap();
        let mut items = Vec::new();
        for item in item_iterator {
            // println!("{:#?}", item);
            items.push(item?);
        }
        Ok(items)
    }
    pub fn list_items(&self) -> Result<Vec<Item>> {
        // let sql = "select * from `todo_items`";
        // and t1.item_delete_flag=0
        let sql ="SELECT t1.item_id,
         t1.item_name, t1.item_type, t2.item_type_name,
         t1.item_desc,t1.item_state,t1.item_priority,t1.item_start,t1.item_end,t1.item_desc_type   FROM todo_items AS t1, item_type AS t2 WHERE t1.item_type=t2.item_type ;";
        let mut stmt = self.con.prepare(sql).unwrap();
        let item_iterator = stmt
            .query_map([], |row| {
                Ok(Item {
                    item_id: row.get(0)?,
                    item_name: row.get(1)?,
                    item_type: row.get::<usize, i32>(2)?.to_string(),
                    item_type_name: row.get(3)?,
                    item_desc: row.get(4)?,
                    item_state: row.get(5)?,
                    item_priority: row.get::<usize, i32>(6)?.to_string(),
                    item_start: row.get(7)?,
                    item_end: row.get(8)?,
                    item_desc_type: row.get(9)?,
                })
            })
            .unwrap();
        let mut items = Vec::new();
        for item in item_iterator {
            // println!("{:#?}", item);
            items.push(item?);
        }
        Ok(items)
    }

    pub fn list_trash_items(&self) -> Result<Vec<Item>> {
        let sql ="SELECT t1.item_id,
         t1.item_name, t1.item_type,
         t1.item_desc,t1.item_state,t1.item_priority,t1.item_start,t1.item_end,t1.item_desc_type   FROM todo_items AS t1 WHERE t1.item_type=1024 ;";
        let mut stmt = self.con.prepare(sql).unwrap();
        let item_iterator = stmt
            .query_map([], |row| {
                Ok(Item {
                    item_id: row.get(0)?,
                    item_name: row.get(1)?,
                    item_type: row.get::<usize, i32>(2)?.to_string(),
                    item_type_name: "废纸篓".to_string(),
                    item_desc: row.get(3)?,
                    item_state: row.get(4)?,
                    item_priority: row.get::<usize, i32>(5)?.to_string(),
                    item_start: row.get(6)?,
                    item_end: row.get(7)?,
                    item_desc_type: row.get(8)?,
                })
            })
            .unwrap();
        let mut items = Vec::new();
        for item in item_iterator {
            // println!("{:#?}", item);
            items.push(item?);
        }
        Ok(items)
    }
}
