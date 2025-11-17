// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod audio_helper;

use std::time::Duration;

use objc2_core_audio::AudioDeviceID;
use tauri::Emitter;

use crate::audio_helper::AudioDevice;

#[tauri::command]
fn get_device_list() -> Vec<AudioDevice> {
  audio_helper::get_device_list()
}

#[tauri::command]
fn set_current_device(device_id: AudioDeviceID, input: bool) -> Result<(), String> {
  audio_helper::set_current_device(device_id, input)
}

#[tauri::command]
fn mute(device_id: AudioDeviceID, mute: bool) -> Result<AudioDevice, String> {
  let result = audio_helper::mute(device_id, mute);
  match result {
    Ok(_) => Ok(audio_helper::create_device(device_id)),
    Err(e) => Err(format!("Failed to mute device: {:?}", e)),
  }
}

#[tauri::command]
fn get_current_device(input: bool) -> Option<AudioDevice> {
  audio_helper::get_current_device(input)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let handle: tauri::AppHandle = app.handle().clone();
      let mut current_input_device = audio_helper::get_current_device(true).unwrap();
      std::thread::spawn(move || loop {
        if audio_helper::get_current_device(true).is_some() {
          let device = audio_helper::get_current_device(true).unwrap();
          if device.id != current_input_device.id {
            handle.emit("input-device-changed", device.clone()).unwrap();
            current_input_device = device;
          }
        }
        std::thread::sleep(Duration::from_secs(10));
      });
      Ok(())
    })
    // .setup(|app| {
    //   #[cfg(desktop)]
    //   {
    //     use tauri_plugin_global_shortcut::{
    //       Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
    //     };
    //     let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::FN), Code::KeyM);
    //     app.handle().plugin(
    //       tauri_plugin_global_shortcut::Builder::new()
    //         .with_handler(move |_app, shortcut, event| {
    //           println!("{:?}", shortcut);
    //           if shortcut == &ctrl_n_shortcut {
    //             match event.state() {
    //               ShortcutState::Pressed => {
    //                 println!("CapsLock-M Pressed!");
    //               }
    //               ShortcutState::Released => {
    //                 println!("CapsLock-M Released!");
    //               }
    //             }
    //           }
    //         })
    //         .build(),
    //     )?;
    //     app.global_shortcut().register(ctrl_n_shortcut)?;
    //   }
    //   Ok(())
    // })
    // .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      get_current_device,
      get_device_list,
      set_current_device,
      mute
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
