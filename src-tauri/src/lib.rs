// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod audio_helper;

use std::{
  sync::{mpsc, Mutex},
  thread,
  time::Duration,
};

use objc2_core_audio::AudioDeviceID;
use tauri::Emitter;
use tauri::Manager;

use crate::audio_helper::{
  create_device, AudioDevice, AudioDeviceType, CurrentDeviceListener, MuteListener,
};

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

struct ListenerState {
  current_device_listener: Mutex<Option<Box<CurrentDeviceListener>>>,
  mute_listeners: Mutex<Vec<Box<MuteListener>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(ListenerState {
      current_device_listener: Mutex::new(None),
      mute_listeners: Mutex::new(Vec::new()),
    })
    .setup(|app| {
      let handle = app.handle().clone();
      let (sender, receiver) = mpsc::channel();
      let (mute_sender, mute_receiver) = mpsc::channel();

      // Store the listener in state so it doesn't get dropped
      let mut listener = Box::new(CurrentDeviceListener::new(sender));
      listener.register()?;
      handle
        .state::<ListenerState>()
        .current_device_listener
        .lock()
        .unwrap()
        .replace(listener);

      // Create mute listeners for all input devices
      let input_devices = audio_helper::get_device_list()
        .into_iter()
        .filter(|d| d.device_type == AudioDeviceType::Input)
        .collect::<Vec<_>>();
      let mut mute_listeners = Vec::new();
      for device in input_devices {
        let mut mute_listener = Box::new(MuteListener::new(device.id, mute_sender.clone()));
        mute_listener.register()?;
        mute_listeners.push(mute_listener);
      }
      handle
        .state::<ListenerState>()
        .mute_listeners
        .lock()
        .unwrap()
        .clear();
      handle
        .state::<ListenerState>()
        .mute_listeners
        .lock()
        .unwrap()
        .extend(mute_listeners);

      thread::spawn(move || {
        // Use `select!`‑style loop if you have multiple receivers.
        loop {
          // Prefer `recv_timeout` to avoid blocking forever.
          match mute_receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(changed_device) => {
              println!("Mute changed: {:?}", changed_device);

              handle.emit("mute-changed", changed_device).unwrap();
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {} // just a timeout, continue
            Err(_) => break,                           // channel closed
          }

          match receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(new_device_id) => {
              println!(
                "Received input device change event for device ID: {:?}",
                new_device_id
              );

              handle
                .emit("input-device-changed", create_device(new_device_id))
                .unwrap();
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {} // just a timeout, continue
            Err(_) => break,                           // channel closed
          }
        }
      });

      Ok(())
    })
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
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
