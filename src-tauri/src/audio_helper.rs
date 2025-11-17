use std::{
  mem,
  ptr::{null, NonNull},
};

use coreaudio::{
  audio_unit::{macos_helpers, Scope},
  Error,
};
use objc2_core_audio::{
  kAudioDevicePropertyMute, kAudioHardwarePropertyDefaultInputDevice,
  kAudioHardwarePropertyDefaultOutputDevice, kAudioObjectPropertyElementMain,
  kAudioObjectPropertyScopeGlobal, kAudioObjectPropertyScopeInput, kAudioObjectSystemObject,
  AudioDeviceID, AudioObjectGetPropertyData, AudioObjectGetPropertyDataSize, AudioObjectID,
  AudioObjectPropertyAddress, AudioObjectSetPropertyData,
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum AudioDeviceType {
  #[serde(rename = "input")]
  Input,
  #[serde(rename = "output")]
  Output,
}

/// Represents an audio device with its properties.
#[derive(Debug, Serialize, Clone)]
pub struct AudioDevice {
  /// The unique identifier of the audio device.
  pub id: u32,

  /// The name of the audio device.
  pub name: String,

  /// The type of the audio device (input or output).
  #[serde(rename = "deviceType")]
  pub device_type: AudioDeviceType,

  /// Indicates whether the device is the current default device.
  #[serde(rename = "isCurrent")]
  pub is_current: bool,

  /// Indicates whether the device is muted.
  #[serde(rename = "isMuted")]
  pub is_muted: bool,
}

/// Creates an `AudioDevice` struct from the given device ID.
///
/// # Arguments
/// * `device_id` - The ID of the audio device.
///
/// # Returns
/// * `AudioDevice` - The created audio device struct.
pub fn create_device(device_id: AudioDeviceID) -> AudioDevice {
  let device_name = macos_helpers::get_device_name(device_id).unwrap_or(String::from("N/A"));
  let is_input =
    macos_helpers::get_audio_device_supports_scope(device_id, Scope::Input).unwrap_or(false);
  let is_current =
    macos_helpers::get_default_device_id(is_input).map_or(false, |id| id == device_id);
  let is_muted = is_muted(device_id).unwrap_or(false);

  AudioDevice {
    id: device_id,
    name: device_name,
    device_type: if is_input {
      AudioDeviceType::Input
    } else {
      AudioDeviceType::Output
    },
    is_current,
    is_muted,
  }
}

/// Retrieves a list of all audio devices on the system.
///
/// # Returns
/// * `Vec<AudioDevice>` - A vector containing all audio devices.
pub fn get_device_list() -> Vec<AudioDevice> {
  let mut devices = Vec::new();
  let device_ids = macos_helpers::get_audio_device_ids();

  if let Ok(device_ids) = device_ids {
    for device_id in device_ids {
      let device = create_device(device_id);
      devices.push(device);
    }
  }
  devices
}

/// Gets the current audio device for input or output.
///
/// # Arguments
/// * `input` - If `true`, gets the current input device; if `false`, gets the current output device.
///
/// # Returns
/// * `Some(AudioDevice)` if a current device is found.
/// * `None` if no current device is found.
pub fn get_current_device(input: bool) -> Option<AudioDevice> {
  let device_id = macos_helpers::get_default_device_id(input)?;

  Some(create_device(device_id))
}

/// Checks if the specified audio device is muted.
///
/// # Arguments
/// * `device_id` - The ID of the audio device to check.
///
/// # Returns
/// * `Ok(bool)` indicating whether the device is muted.
/// * `Err(String)` if an error occurred.
fn is_muted(device_id: AudioDeviceID) -> Result<bool, String> {
  let property_address = AudioObjectPropertyAddress {
    mSelector: kAudioDevicePropertyMute,
    mScope: kAudioObjectPropertyScopeInput,
    mElement: kAudioObjectPropertyElementMain,
  };

  let mut data_size = 0u32;
  let status = unsafe {
    AudioObjectGetPropertyDataSize(
      device_id,
      NonNull::from(&property_address),
      0,
      null(),
      NonNull::from(&mut data_size),
    )
  };

  if (Error::from_os_status(status).is_err()) || data_size == 0 {
    return Err("Failed to get mute property data size".to_string());
  }

  let mut is_muted: u32 = 0;

  let status = unsafe {
    AudioObjectGetPropertyData(
      device_id,
      NonNull::from(&property_address),
      0,
      null(),
      NonNull::from(&data_size),
      NonNull::from(&mut is_muted).cast(),
    )
  };

  let result = Error::from_os_status(status);
  match result {
    Ok(_) => Ok(is_muted != 0),
    Err(e) => Err(format!("Failed to get mute status: {:?}", e)),
  }
}

/// Mutes or unmutes the specified audio device.
///
/// # Arguments
///
/// * `device_id` - The ID of the audio device to mute or unmute.
/// * `mute` - If `true`, the device will be muted; if `false`, it will be unmuted.
///
/// # Returns
///
/// * `Ok(())` if the operation was successful.
/// * `Err(String)` if an error occurred.
pub fn mute(device_id: AudioDeviceID, mute: bool) -> Result<(), String> {
  let property_address = AudioObjectPropertyAddress {
    mSelector: kAudioDevicePropertyMute,
    mScope: kAudioObjectPropertyScopeInput,
    mElement: 0,
  };

  let mute: u32 = if mute { 1 } else { 0 }; // 1 to mute, 0 to unmute
  let data_size = mem::size_of::<u32>() as u32;
  let status = unsafe {
    AudioObjectSetPropertyData(
      device_id as AudioObjectID,
      NonNull::from(&property_address),
      0,
      null(),
      data_size,
      NonNull::from(&mute).cast(),
    )
  };
  let result = Error::from_os_status(status);
  match result {
    Ok(_) => Ok(()),
    Err(e) => Err(format!("Failed to mute device: {:?}", e)),
  }
}

/// Sets the current audio device for input or output.
///
/// # Arguments
/// * `device_id` - The ID of the audio device to set as current.
/// * `input` - If `true`, sets the device as the current input device; if `false`, as the current output device.
///
/// # Returns
/// * `Ok(())` if the operation was successful.
/// * `Err(String)` if an error occurred.
pub fn set_current_device(device_id: AudioDeviceID, input: bool) -> Result<(), String> {
  let property_address = AudioObjectPropertyAddress {
    mSelector: if input {
      kAudioHardwarePropertyDefaultInputDevice
    } else {
      kAudioHardwarePropertyDefaultOutputDevice
    },
    mScope: kAudioObjectPropertyScopeGlobal,
    mElement: kAudioObjectPropertyElementMain,
  };

  let data_size = mem::size_of::<AudioDeviceID>() as u32;
  let status = unsafe {
    AudioObjectSetPropertyData(
      kAudioObjectSystemObject as AudioObjectID,
      NonNull::from(&property_address),
      0,
      null(),
      data_size,
      NonNull::from(&device_id).cast(),
    )
  };
  let result = Error::from_os_status(status);
  match result {
    Ok(_) => Ok(()),
    Err(e) => Err(format!("Failed to set device: {:?}", e)),
  }
}
