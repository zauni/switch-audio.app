# Switch Audio App

> macOS app to quickly switch between headset and built-in input/output audio devices and also quickly mute them.
> With corresponding menubar icons to see which device is currently used

# Todo

- Instead of polling, use `AudioObjectAddPropertyListener` to check if input/output device changed and if device is muted
- Add [global hotkeys](https://v2.tauri.app/plugin/global-shortcut/)
- Adjust [app icon](https://tauri.app/develop/icons/)
- Remove window
- Remove unused stuff
- Support bluetooth device `Pro X 2`
- Multi-Language support

# Long Term Goals

- Settings UI to adjust supported devices
- Adjust menubar icons to be bigger and have a red/yellow warning sign if muted
- Show current input/output devices in menu

## Getting battery status of headset

### Option 1: Logitech G Hub Websocket connection

see [Rust library](https://github.com/Fuwn/elem/blob/main/src/logitech.rs)

### Option 2: Logitech G Hub SDK:

> **Yes** – Logitech provides the **Logitech G Hub SDK** (formerly the Logitech G Hub API) for programmatic access to G‑series devices, including the Pro X 2 headset.
>
> ## macOS support
>
> - The SDK is distributed as a **macOS‑compatible framework** (`LogitechGHub.framework`).
> - It works with macOS 10.15 (Catalina) and later; you’ll need Xcode 13+ to build against it.
>
> ## What you can query
>
> | Feature                     | Availability via SDK       | Typical call                                              |
> | --------------------------- | -------------------------- | --------------------------------------------------------- |
> | Battery level               | ✅ (via `GetBatteryLevel`) | `LGHDeviceBatteryInfo battery = [device getBatteryInfo];` |
> | Equalizer / preset settings | ✅ (via `GetEQSettings`)   | `LGHEQProfile *profile = [device getCurrentEQ];`          |
> | Mic gain, sidetone, etc.    | ✅                         | `LGHDeviceSettings *settings = [device getSettings];`     |
> | Firmware version            | ✅                         | `NSString *fw = [device firmwareVersion];`                |
>
> ## Getting started
>
> 1. **Download the SDK** from the Logitech G Hub developer portal (requires a free developer account).
> 2. Add `LogitechGHub.framework` to your Xcode project and import it:
>
>    ```swift
>    import LogitechGHub
>    ```
>
> 3. Discover devices:
>
>    ```swift
>    let hub = LGHManager.shared()
>    hub.scanForDevices()
>    let headset = hub.connectedDevices.first { $0.productName == "Logitech Pro X 2" }
>    ```
>
> 4. Query battery:
>
>    ```swift
>    if let batteryInfo = headset?.getBatteryInfo() {
>        print("Battery: \(batteryInfo.percentage)%")
>    }
>    ```
>
> ## Limitations
>
> - The SDK works only when **Logitech G Hub** is running; it cannot communicate with the headset if the daemon is stopped.
> - Some advanced settings (e.g., custom DSP profiles) are read‑only via the API.
> - The SDK is **not officially supported on Linux**; macOS and Windows are the only guaranteed platforms.
>
> ## Alternative approaches
>
> - **HID‑raw**: The headset exposes a standard HID interface; you can read battery status via `ioreg` or a custom HID library, but this requires reverse‑engineering and lacks official documentation.
> - **Bluetooth LE**: If the headset is used wirelessly, you can query the battery characteristic via CoreBluetooth, though you won’t get the full G‑Hub settings.
>
> Overall, the Logitech G Hub SDK is the recommended, officially supported way to programmatically read battery level and other settings for the Pro X 2 on macOS.

## Use menubar icon to show a kind of slider how much voice is detected

> You cannot create a dynamic, real-time “slider” or meter using a standard template icon in the macOS menu bar. Template icons are static, monochrome images (PDF or PNG) that adapt to light/dark mode, but they do not support animation or dynamic updates within the icon itself.
>
> However, you can simulate this effect by programmatically updating the menu bar icon with different images representing the current pitch or level. For example:
>
> Pre-generate a set of template images (e.g., 10 icons showing different slider positions).
> As the pitch changes, swap the menu bar icon to the corresponding image.
> Summary:
>
> Directly animated or dynamic template icons: Not possible.
> Swapping static icons to simulate a slider: Possible, but you must manage and update the icon images yourself.
> Tip:
> If you want color or more complex visuals, use non-template icons, but you lose automatic adaptation to system appearance.

## Getting info if it is connected via bluetooth or lightning adapter

> In macOS Core Audio, you can determine how an audio device is connected (e.g., Bluetooth, USB, built-in) by querying the device’s transport type property.
>
> Use the property kAudioDevicePropertyTransportType with the AudioObject API. This property returns an UInt32 value that indicates the transport type.
>
> Common transport type values:
>
> kAudioDeviceTransportTypeBluetooth (0x62747620, 'btv ')
> kAudioDeviceTransportTypeUSB (0x75736220, 'usb ')
> kAudioDeviceTransportTypeBuiltIn (0x62696e20, 'bin ')
> kAudioDeviceTransportTypeAggregate (0x61676720, 'agg ')
> kAudioDeviceTransportTypeVirtual (0x76697274, 'virt')
> Example in Swift:
>
> ```swift
> import CoreAudio
>
> func getTransportType(for deviceID: AudioDeviceID) -> UInt32? {
>     var transportType: UInt32 = 0
>     var propertyAddress = AudioObjectPropertyAddress(
>         mSelector: kAudioDevicePropertyTransportType,
>         mScope: kAudioObjectPropertyScopeGlobal,
>         mElement: kAudioObjectPropertyElementMain
>     )
>     var dataSize = UInt32(MemoryLayout<UInt32>.size)
>     let status = AudioObjectGetPropertyData(
>         deviceID,
>         &propertyAddress,
>         0,
>         nil,
>         &dataSize,
>         &transportType
>     )
>     return (status == noErr) ? transportType : nil
> }
> ```
>
> You can then compare the returned value to the constants above to determine the connection type.
