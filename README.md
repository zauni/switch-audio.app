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

# Long Term Goals

- Settings UI to adjust supported devices
