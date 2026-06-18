import { invoke } from "@tauri-apps/api/core";
import type { HistoryItem, Settings } from "../types";

export const getSettings = (): Promise<Settings> => invoke("get_settings");

export const saveSettings = (settings: Settings): Promise<void> =>
  invoke("save_settings", { settings });

export const captureAndUpload = (): Promise<HistoryItem> =>
  invoke("capture_and_upload");

export const listHistory = (): Promise<HistoryItem[]> => invoke("list_history");

export const clearHistory = (): Promise<void> => invoke("clear_history");

export const copyToClipboard = (text: string): Promise<void> =>
  invoke("copy_to_clipboard", { text });
