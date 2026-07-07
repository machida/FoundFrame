mod application;
mod commands;
mod domain;
mod dto;
mod errors;
mod filesystem;
mod keychain;
mod persistence;
mod prompt_engine;
mod providers;
mod review;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::system::app_bootstrap_status,
            commands::system::dictionary_debug_status,
            commands::system::setup_bootstrap_data,
            commands::system::create_roll,
            commands::system::resolve_setup_preview,
            commands::system::process_contact_sheet_roll,
            commands::system::roll_detail,
            commands::system::select_frame_and_generate_alternate_take,
            commands::system::settings_snapshot,
            commands::system::save_provider_api_key,
            commands::system::clear_provider_api_key,
            commands::system::test_provider_connection,
            commands::system::recent_rolls,
            commands::system::set_frame_favorite,
            commands::system::presets,
            commands::system::save_preset,
            commands::system::delete_preset,
            commands::system::rename_preset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
