import { useCallback, useEffect, useState } from "react";
import "./App.css";
import { HistoryList } from "./components/HistoryList";
import { SettingsForm } from "./components/SettingsForm";
import { captureAndUpload, clearHistory, getSettings, listHistory } from "./lib/ipc";
import type { HistoryItem, Settings } from "./types";

function App() {
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [configured, setConfigured] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [busy, setBusy] = useState(false);
  const [toast, setToast] = useState<string>("");

  const notify = useCallback((message: string) => {
    setToast(message);
    window.setTimeout(() => setToast(""), 2500);
  }, []);

  const refreshHistory = useCallback(() => {
    listHistory().then(setHistory).catch((e) => notify(String(e)));
  }, [notify]);

  const isComplete = (s: Settings) =>
    s.base_url.trim() !== "" && s.user.trim() !== "" && s.pass.trim() !== "";

  useEffect(() => {
    getSettings()
      .then((s) => {
        const ok = isComplete(s);
        setConfigured(ok);
        setShowSettings(!ok);
      })
      .catch(() => setShowSettings(true));
    refreshHistory();
  }, [refreshHistory]);

  const handleCapture = async () => {
    setBusy(true);
    try {
      await captureAndUpload();
      notify("Captured and copied URL to clipboard");
      refreshHistory();
    } catch (e) {
      const msg = String(e);
      // Silently ignore cancellation.
      if (!msg.includes("cancel")) notify(msg);
    } finally {
      setBusy(false);
    }
  };

  const handleClear = async () => {
    await clearHistory();
    refreshHistory();
  };

  const handleSaved = (s: Settings) => {
    setConfigured(isComplete(s));
    setShowSettings(false);
  };

  return (
    <main className="container">
      <header>
        <h1>sizuku</h1>
        <button
          type="button"
          className="link"
          onClick={() => setShowSettings((v) => !v)}
        >
          {showSettings ? "Close" : "Settings"}
        </button>
      </header>

      {showSettings && <SettingsForm onSaved={handleSaved} />}

      <button
        type="button"
        className="capture"
        disabled={busy || !configured}
        onClick={handleCapture}
      >
        {busy ? "Processing…" : "Capture a region"}
      </button>
      {!configured && (
        <p className="hint">Save your connection settings first.</p>
      )}

      <HistoryList items={history} onClear={handleClear} onNotify={notify} />

      {toast && <div className="toast">{toast}</div>}
    </main>
  );
}

export default App;
