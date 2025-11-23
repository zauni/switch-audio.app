import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  IconMenuItem,
  Menu,
  MenuItem,
  PredefinedMenuItem,
} from "@tauri-apps/api/menu";
import { TrayIcon } from "@tauri-apps/api/tray";
import { toggleAudioDevice, toggleMute, type Device } from "./audio-helper";
import { register } from "@tauri-apps/plugin-global-shortcut";
import { resolveResource } from "@tauri-apps/api/path";

export async function initTray() {
  await register("CommandOrControl+Shift+N", (evt) => {
    if (evt.state === "Pressed") toggleAudioDevice();
  });
  await register("CommandOrControl+Shift+M", (evt) => {
    if (evt.state === "Pressed") toggleMute();
  });

  const headsetMutedImage = await resolveResource("assets/headset-muted.png");
  const headsetImage = await resolveResource("assets/headset.png");
  const laptopMutedImage = await resolveResource("assets/laptop-muted.png");
  const laptopImage = await resolveResource("assets/laptop.png");
  const micMutedImage = await resolveResource("assets/mic-muted.png");
  const micImage = await resolveResource("assets/mic.png");
  const switchImage = await resolveResource("assets/switch.png");

  const muteMenuItem = await IconMenuItem.new({
    icon: micMutedImage,
    id: "mute",
    text: "Mute",
    accelerator: "CommandOrControl+Shift+M",
    action: toggleMute,
  });

  const currentDeviceMenuItem = await MenuItem.new({
    id: "current-device",
    text: "Aktuelles Gerät: ...",
    enabled: false,
  });

  const tray = await TrayIcon.new({
    tooltip: "Switch Audio",
    menu: await Menu.new({
      items: [
        await IconMenuItem.new({
          icon: switchImage,
          id: "toggle-audio",
          text: "Audio-Gerät wechseln",
          accelerator: "CommandOrControl+Shift+N",
          action: toggleAudioDevice,
        }),
        muteMenuItem,
        await PredefinedMenuItem.new({
          text: "separator-text",
          item: "Separator",
        }),
        currentDeviceMenuItem,
        await PredefinedMenuItem.new({
          text: "Beenden",
          item: "Quit",
        }),
      ],
    }),
  });

  const updateIcon = async () => {
    const device = await invoke<Device>("get_current_device", {
      input: true,
    });
    let icon = device.isMuted ? laptopMutedImage : laptopImage;
    if (device.name === "PRO X 2 LIGHTSPEED") {
      icon = device.isMuted ? headsetMutedImage : headsetImage;
    }
    await tray.setIcon(icon);
    await tray.setIconAsTemplate(true);

    await muteMenuItem.setIcon(device.isMuted ? micImage : micMutedImage);
    await muteMenuItem.setText(device.isMuted ? "Aktivieren" : "Stummschalten");

    await currentDeviceMenuItem.setText(`Aktuelles Gerät: ${device.name}`);
  };

  console.log(await invoke<Device[]>("get_device_list"));

  await updateIcon();

  listen("input-device-changed", async () => {
    await updateIcon();
  });

  listen("mute-changed", async () => {
    await updateIcon();
  });
}
