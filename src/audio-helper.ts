import { invoke } from "@tauri-apps/api/core";

export type Device = {
  id: number;
  name: string;
  deviceType: "input" | "output";
  isCurrent: boolean;
  isMuted: boolean;
};

/**
 * Toggles the current audio device between "PRO X 2 LIGHTSPEED" and the MacBook's built-in audio devices.
 * @returns The newly set current input device.
 */
export async function toggleAudioDevice(): Promise<Device> {
  const deviceList = await invoke<Device[]>("get_device_list");
  const currentInput = deviceList.find(
    (d) => d.isCurrent && d.deviceType === "input"
  );
  const macBookInput = deviceList.find(
    (d) => d.name === "MacBook Pro-Mikrofon"
  );
  const macBookOutput = deviceList.find(
    (d) => d.name === "MacBook Pro-Lautsprecher"
  );
  const headsetInput = deviceList.find(
    (d) => d.name === "PRO X 2 LIGHTSPEED" && d.deviceType === "input"
  );
  const headsetOutput = deviceList.find(
    (d) => d.name === "PRO X 2 LIGHTSPEED" && d.deviceType === "output"
  );

  if (
    currentInput?.name === "PRO X 2 LIGHTSPEED" &&
    macBookInput &&
    macBookOutput
  ) {
    await invoke("set_current_device", {
      deviceId: macBookInput.id,
      input: true,
    });
    await invoke("set_current_device", {
      deviceId: macBookOutput.id,
      input: false,
    });
    return macBookInput;
  } else if (headsetInput && headsetOutput) {
    await invoke("set_current_device", {
      deviceId: headsetInput.id,
      input: true,
    });
    await invoke("set_current_device", {
      deviceId: headsetOutput.id,
      input: false,
    });
    return headsetInput;
  }

  return currentInput!;
}

export async function toggleMute(): Promise<Device> {
  const device = await invoke<Device>("get_current_device", {
    input: true,
  });

  const result = await invoke<Device>("mute", {
    deviceId: device.id,
    mute: !device.isMuted,
  });
  return result;
}
