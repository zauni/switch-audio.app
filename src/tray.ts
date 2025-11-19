import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  IconMenuItem,
  Menu,
  NativeIcon,
  PredefinedMenuItem,
} from "@tauri-apps/api/menu";
import { TrayIcon } from "@tauri-apps/api/tray";
import { toggleAudioDevice, toggleMute, type Device } from "./audio-helper";
// import { register } from "@tauri-apps/plugin-global-shortcut";
import { resolveResource } from "@tauri-apps/api/path";

export async function initTray() {
  // await register("Function+M", () => {
  //   console.log("Shortcut triggered");
  // });

  const headsetMutedImage = await resolveResource("assets/headset-muted.png");
  const headsetImage = await resolveResource("assets/headset.png");
  const laptopMutedImage = await resolveResource("assets/laptop-muted.png");
  const laptopImage = await resolveResource("assets/laptop.png");

  const tray = await TrayIcon.new({
    // `icon` expects a base64 string or a path relative to the `src-tauri` folder
    icon: laptopImage,
    tooltip: "Switch Audio",
    iconAsTemplate: true, // for macOS dark mode support
    menu: await Menu.new({
      items: [
        await IconMenuItem.new({
          icon: NativeIcon.Refresh,
          id: "toggle-audio",
          text: "Toggle Audio Device",
          // accelerator: "CommandOrControl+Shift+C",
          action: async () => {
            try {
              await toggleAudioDevice();
            } catch (error) {
              console.error("Error toggling audio device:", error);
            }
          },
        }),
        await IconMenuItem.new({
          icon: NativeIcon.InvalidDataFreestanding,
          id: "mute",
          text: "Mute/Unmute",
          action: async () => {
            try {
              const currentDevice = await toggleMute();

              await updateIcon(currentDevice);
            } catch (error) {
              console.error("Error muting audio device:", error);
            }
          },
        }),
        await PredefinedMenuItem.new({
          text: "separator-text",
          item: "Separator",
        }),
        await PredefinedMenuItem.new({
          text: "Beenden",
          item: "Quit",
        }),
      ],
    }),
  });

  const updateIcon = async (device: Device) => {
    let icon = device?.isMuted ? laptopMutedImage : laptopImage;
    if (device?.name === "PRO X 2 LIGHTSPEED") {
      icon = device?.isMuted ? headsetMutedImage : headsetImage;
    }
    await tray.setIcon(icon);
    await tray.setIconAsTemplate(true);
  };

  console.log(await invoke<Device[]>("get_device_list"));
  const currentDevice = await invoke<Device>("get_current_device", {
    input: true,
  });

  await updateIcon(currentDevice);

  listen<Device>("input-device-changed", async (evt) => {
    console.log("input-device-changed event received", evt.payload);
    await updateIcon(evt.payload);
  });
}
