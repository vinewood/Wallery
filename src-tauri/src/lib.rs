pub mod cache;
pub mod commands;
pub mod config;
pub mod favorites;
pub mod scheduler;
pub mod sources;
pub mod wallpaper_manager;

use scheduler::Scheduler;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tokio::sync::Mutex;

pub struct AppState {
    pub config: Mutex<config::WalleryConfig>,
    pub scheduler: Mutex<Option<Scheduler>>,
}

#[tauri::command]
fn open_settings(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn hide_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Clean up stale WebView2 lockfile that can cause ERR_CONNECTION_REFUSED.
#[allow(unused)]
fn cleanup_webview_lock() {
    #[cfg(target_os = "windows")]
    {
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let lockfile = std::path::PathBuf::from(local_app_data)
                .join("com.wallery.app")
                .join("EBWebView")
                .join("lockfile");
            if lockfile.exists() {
                log::info!("Removing stale WebView2 lockfile: {:?}", lockfile);
                let _ = std::fs::remove_file(&lockfile);
            }
        }
    }
}

pub fn run() {
    env_logger::init();

    // Clean up stale WebView2 lockfile to prevent startup crash
    cleanup_webview_lock();

    // Load config or create default
    let cfg = config::WalleryConfig::load().unwrap_or_default();
    let _cfg_path = config::get_config_path();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .manage(AppState {
            config: Mutex::new(cfg),
            scheduler: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            open_settings,
            hide_window,
            commands::wallpaper_cmds::next_wallpaper,
            commands::wallpaper_cmds::get_current_wallpaper_info,
            commands::wallpaper_cmds::save_current_wallpaper,
            commands::wallpaper_cmds::copy_image_url,
            commands::wallpaper_cmds::download_wallpaper_url,
            commands::settings_cmds::get_settings,
            commands::settings_cmds::update_settings,
            commands::settings_cmds::get_schedule,
            commands::settings_cmds::set_schedule,
            commands::settings_cmds::get_user_categories,
            commands::settings_cmds::add_user_category,
            commands::settings_cmds::remove_user_category,
            commands::settings_cmds::set_source_enabled,
            commands::settings_cmds::set_api_key,
            commands::settings_cmds::set_auto_start,
            commands::settings_cmds::get_auto_start,
            commands::settings_cmds::get_download_settings,
            commands::settings_cmds::set_download_path,
            commands::settings_cmds::set_open_folder_after_download,
            commands::settings_cmds::open_download_folder,
            commands::settings_cmds::set_last_selected_source,
            commands::settings_cmds::get_last_selected_source,
            commands::source_cmds::get_sources_status,
            commands::source_cmds::get_source_hot_categories,
            commands::source_cmds::browse_source,
            commands::source_cmds::set_wallpaper_from,
            commands::source_cmds::get_wallpaper_cache,
            commands::favorites_cmds::add_favorite,
            commands::favorites_cmds::remove_favorite,
            commands::favorites_cmds::get_favorites,
            commands::favorites_cmds::is_favorited,
            commands::update_cmds::check_update,
            commands::update_cmds::download_update,
            commands::update_cmds::apply_update,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Build tray menu — Chinese labels
            let next_wallpaper =
                MenuItem::with_id(app, "next", "下一张壁纸", true, None::<&str>)?;
            let skip_today =
                MenuItem::with_id(app, "skip", "跳过今天", true, None::<&str>)?;
            let sep1 = tauri::menu::PredefinedMenuItem::separator(app)?;
            let copy_url =
                MenuItem::with_id(app, "copy_url", "复制图片链接", true, None::<&str>)?;
            let save_img =
                MenuItem::with_id(app, "save", "保存当前壁纸", true, None::<&str>)?;
            let open_src =
                MenuItem::with_id(app, "open_src", "打开来源页面", true, None::<&str>)?;
            let sep2 = tauri::menu::PredefinedMenuItem::separator(app)?;
            let settings =
                MenuItem::with_id(app, "settings", "设置…", true, None::<&str>)?;
            let sep3 = tauri::menu::PredefinedMenuItem::separator(app)?;
            let quit = MenuItem::with_id(app, "quit", "退出 Wallery", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &next_wallpaper,
                    &skip_today,
                    &sep1,
                    &copy_url,
                    &save_img,
                    &open_src,
                    &sep2,
                    &settings,
                    &sep3,
                    &quit,
                ],
            )?;

            // Build tray icon — load from default or skip icon
            let mut tray_builder = TrayIconBuilder::new()
                .tooltip("Wallery 幕间")
                .menu(&menu);

            if let Some(ico) = app.default_window_icon().cloned() {
                tray_builder = tray_builder.icon(ico);
            }

            let _tray = tray_builder
                .on_menu_event(move |app, event| {
                    let id = event.id.as_ref();
                    match id {
                        "next" => {
                            let handle = app.clone();
                            tauri::async_runtime::spawn(async move {
                                if let Err(e) =
                                    commands::wallpaper_cmds::do_next_wallpaper(&handle).await
                                {
                                    log::error!("Failed to change wallpaper: {}", e);
                                }
                            });
                        }
                        "skip" => {
                            log::info!("Skip today triggered");
                        }
                        "copy_url" => {
                            log::info!("Copy URL triggered");
                        }
                        "save" => {
                            log::info!("Save triggered");
                        }
                        "open_src" => {
                            log::info!("Open source page triggered");
                        }
                        "settings" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // Start scheduler
            let handle = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                let mut scheduler = Scheduler::new(handle.clone());
                scheduler.start().await;
            });

            // Show window on first launch
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();

                // Close window → hide to tray instead of quitting
                let win = window.clone();
                let _ = window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        let _ = win.hide();
                        api.prevent_close();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
