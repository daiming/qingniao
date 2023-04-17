// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
// 用户的结构体
#[derive(Serialize, Deserialize)]
struct User {
    mac: String,
    ipaddr: String,
    nickname: String,
    avatar: String,
    msg_count: u32,
}
// 消息结构体
#[derive(Serialize, Deserialize)]
struct Message {
    mac: String,
    message: String,
    msg_time: String,
    is_self: bool,
}

#[tauri::command]
fn get_user_list() -> Vec<User> {
    let user_list = vec![
        User {
            mac: "00:00:00:00:00:00".to_string(),
            ipaddr: "192.168.0.1".to_string(),
            nickname: "如来".to_string(),
            avatar: "avatar01".to_string(),
            msg_count: 5,
        },
        User {
            mac: "00:00:00:00:00:00".to_string(),
            ipaddr: "192.168.0.2".to_string(),
            nickname: "老子".to_string(),
            avatar: "avatar02".to_string(),
            msg_count: 2,
        },
        User {
            mac: "00:00:00:00:00:00".to_string(),
            ipaddr: "192.168.0.3".to_string(),
            nickname: "孔子".to_string(),
            avatar: "avatar03".to_string(),
            msg_count: 0,
        },
        User {
            mac: "00:00:00:00:00:00".to_string(),
            ipaddr: "192.168.0.4".to_string(),
            nickname: "庄子".to_string(),
            avatar: "avatar04".to_string(),
            msg_count: 999,
        },
    ];
    user_list
}

#[tauri::command]
fn get_user_message(window: tauri::Window, mac: String) -> Vec<Message> {
    let message_list = vec![
        Message {
            mac: "00:00:00:00:00:00".to_string(),
            message: "打印人（患者）身份识别方式可多样化：二代证、就诊卡、居民健康卡、注册手机号、指纹等。可指定打印代理人，代理人识别及权限同打印人。".to_string(),
            msg_time: "2023-02-02 12:32:21".to_string(),
            is_self: false,
        },
        Message {
            mac: "00:00:00:00:00:00".to_string(),
            message: "3.打印范围可按身份角色指定。系统根据授权的资料、密级等判断，生成允许患者打印的资料。".to_string(),
            msg_time: "2023-02-02 12:32:21".to_string(),
            is_self: false,
        },
        Message {
            mac: "00:00:00:00:00:00".to_string(),
            message: "支持患者需要邮寄或邮件资料，病案主管部门可以导出资料进行刻盘，在网路环境允许的情况下，也可直接生成附件，发送邮件。".to_string(),
            msg_time: "2023-02-02 12:32:21".to_string(),
            is_self: true,
        },
        Message {
            mac: "00:00:00:00:00:00".to_string(),
            message: "1.病案封存通过系统赋予某种角色权限完成。并能记录封存现场相关信息，如：封存现场参与人员、确认指纹信息、留存照片等。".to_string(),
            msg_time: "2023-02-02 12:32:21".to_string(),
            is_self: true,
        },
        Message {
            mac: "00:00:00:00:00:00".to_string(),
            message: "拷贝输出".to_string(),
            msg_time: "2023-02-02 12:32:21".to_string(),
            is_self: false,
        },
    ];

    window.set_title(&mac).unwrap();
    message_list
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_user_list,get_user_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
