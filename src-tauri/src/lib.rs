use tauri::{Emitter, Manager};
use std::sync::Mutex;
use std::time::Duration;

mod config;
mod timer;
mod cycle;
mod tray;
mod bird;
mod commands;

use config::AppConfig;
use timer::{TimerManager, SessionType};
use cycle::WorkCycle;

pub struct AppState {
    pub timer: TimerManager,
    pub cycle: Mutex<WorkCycle>,
    pub config: Mutex<AppConfig>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            let config = AppConfig::load(app.handle());
            let pomodoros_per_cycle = config.pomodoros_per_cycle;

            app.manage(AppState {
                timer: TimerManager::new(),
                cycle: Mutex::new(WorkCycle::new(pomodoros_per_cycle)),
                config: Mutex::new(config),
            });

            // Setup tray
            tray::create_tray(app.handle())?;

            // Intercept close - hide instead of quit
            if let Some(window) = app.get_webview_window("main") {
                let win = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = win.hide();
                    }
                });
            }

            // Timer tick loop
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(Duration::from_secs(1));
                    let state = handle.state::<AppState>();
                    if let Some(timer_state) = state.timer.tick() {
                        let _ = handle.emit("timer:tick", &timer_state);

                        if timer_state.status == timer::SessionStatus::Completed {
                            let _ = handle.emit("timer:complete", &timer_state);

                            // Update cycle
                            let session_type = state.timer.session_type();
                            if session_type.as_ref() == Some(&SessionType::Work) {
                                let mut cycle = state.cycle.lock().unwrap();
                                cycle.record_completed();
                                let _ = handle.emit("cycle:updated", cycle.state());

                                // Show bird animation
                                let config = state.config.lock().unwrap();
                                if config.animation_enabled {
                                    let _ = bird::show_bird(&handle);
                                }
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_timer,
            commands::pause_timer,
            commands::resume_timer,
            commands::stop_timer,
            commands::get_config,
            commands::update_config,
            commands::get_cycle_state,
            commands::open_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
