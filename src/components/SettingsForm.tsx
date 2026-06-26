import { useEffect, useState } from "react";
import { getSettings, saveSettings } from "../lib/ipc";
import type { Settings } from "../types";

type Props = {
  onSaved: (settings: Settings) => void;
};

export function SettingsForm({ onSaved }: Props) {
  const [settings, setSettings] = useState<Settings>({
    base_url: "",
    user: "",
    pass: "",
  });
  const [status, setStatus] = useState<string>("");

  useEffect(() => {
    getSettings()
      .then(setSettings)
      .catch((e) => setStatus(String(e)));
  }, []);

  const update = (patch: Partial<Settings>) =>
    setSettings((prev) => ({ ...prev, ...patch }));

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await saveSettings(settings);
      setStatus("Saved");
      onSaved(settings);
    } catch (err) {
      setStatus(`Failed to save: ${String(err)}`);
    }
  };

  return (
    <form className="settings" onSubmit={handleSave}>
      <h2>Connection settings</h2>
      <label>
        Worker URL
        <input
          type="url"
          placeholder="https://r2-image-worker.example.workers.dev"
          value={settings.base_url}
          onChange={(e) => update({ base_url: e.target.value })}
        />
      </label>
      <label>
        Username
        <input
          type="text"
          value={settings.user}
          onChange={(e) => update({ user: e.target.value })}
        />
      </label>
      <label>
        Password
        <input
          type="password"
          value={settings.pass}
          onChange={(e) => update({ pass: e.target.value })}
        />
      </label>
      <button type="submit">Save settings</button>
      {status && <p className="status">{status}</p>}
    </form>
  );
}
