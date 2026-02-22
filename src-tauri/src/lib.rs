//! WriteDo - 码字待办
//!
//! macOS 桌面端任务提醒 + 写作练习工具。
//! 本文件是 Tauri 应用的入口，负责初始化数据库、注册插件和 Commands。

mod errors;
mod models;
mod db;
mod commands;
mod plan_generator;

use std::sync::Mutex;
use rusqlite::Connection;
use tauri::Manager;

use commands::task_cmd;
use commands::plan_cmd;
use commands::writing_cmd;
use commands::import_cmd;

/// 初始化数据库连接
fn init_db() -> Connection {
    let app_dir = dirs_next::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("com.terrell.writedo");

    // 确保目录存在
    std::fs::create_dir_all(&app_dir).expect("无法创建应用数据目录");

    let db_path = app_dir.join(db::init::DB_NAME);
    let conn = Connection::open(&db_path)
        .unwrap_or_else(|e| panic!("无法打开数据库 {:?}: {}", db_path, e));

    // 初始化表结构
    db::init::init_database(&conn).expect("数据库初始化失败");

    conn
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let conn = init_db();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(conn))
        .setup(|app| {
            // ── 系统托盘菜单 ──
            use tauri::menu::{MenuBuilder, MenuItemBuilder};
            use tauri::tray::TrayIconBuilder;

            let show_item = MenuItemBuilder::with_id("show", "显示主窗口").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出 WriteDo").build(app)?;

            let tray_menu = MenuBuilder::new(app)
                .items(&[&show_item, &quit_item])
                .build()?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().unwrap())
                .tooltip("WriteDo - 码字待办")
                .menu(&tray_menu)
                .on_menu_event(|app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(win) = app.get_webview_window("main") {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    use tauri::tray::TrayIconEvent;
                    if let TrayIconEvent::Click { button, button_state, .. } = event {
                        if button == tauri::tray::MouseButton::Left
                            && button_state == tauri::tray::MouseButtonState::Up
                        {
                            if let Some(win) = tray.app_handle().get_webview_window("main") {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 任务管理
            task_cmd::get_tasks,
            task_cmd::get_today_tasks,
            task_cmd::create_task,
            task_cmd::update_task,
            task_cmd::delete_task,
            task_cmd::toggle_task,
            // 写作计划
            plan_cmd::get_plans,
            plan_cmd::get_plan_detail,
            plan_cmd::get_today_writing,
            plan_cmd::import_plan,
            plan_cmd::generate_plan,
            plan_cmd::update_plan,
            plan_cmd::update_plan_status,
            plan_cmd::delete_plan,
            // 写作记录
            writing_cmd::save_writing,
            writing_cmd::get_writings,
            writing_cmd::get_writing_stats,
            writing_cmd::get_heatmap,
            writing_cmd::get_writing_detail,
            // 批量导入
            import_cmd::import_tasks_json,
            import_cmd::import_plan_json,
            import_cmd::import_plan_markdown,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
