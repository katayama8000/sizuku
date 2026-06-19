import { useCallback, useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import "./App.css";
import { HistoryList } from "./components/HistoryList";
import { SettingsForm } from "./components/SettingsForm";
import { clearHistory, getSettings, listHistory } from "./lib/ipc";
import type { HistoryItem, Settings } from "./types";

type View = "main" | "settings";

function App() {
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [configured, setConfigured] = useState(false);
  const [view, setView] = useState<View>("main");
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
        // First run (not configured yet): open the settings screen directly.
        if (!ok) setView("settings");
      })
      .catch(() => setView("settings"));
    refreshHistory();
  }, [refreshHistory]);

  // React to captures triggered from the tray icon or the global shortcut.
  useEffect(() => {
    const unlisten = Promise.all([
      listen("history-updated", () => {
        notify("Captured and copied URL to clipboard");
        refreshHistory();
      }),
      listen<string>("capture-error", (e) => notify(e.payload)),
    ]);
    return () => {
      unlisten.then((fns) => fns.forEach((fn) => fn()));
    };
  }, [notify, refreshHistory]);

  const handleClear = async () => {
    await clearHistory();
    refreshHistory();
  };

  const handleSaved = (s: Settings) => {
    setConfigured(isComplete(s));
    setView("main");
  };

  return (
    <main className="container">
      <header>
        <h1>sizuku</h1>
        {view === "main" ? (
          <button type="button" className="link" onClick={() => setView("settings")}>
            Settings
          </button>
        ) : (
          <button
            type="button"
            className="link"
            onClick={() => setView("main")}
            disabled={!configured}
          >
            ← Back
          </button>
        )}
      </header>

      {view === "settings" ? (
        <SettingsForm onSaved={handleSaved} />
      ) : (
        <>
          <section className="howto">
            <h2>How to capture</h2>
            <ol>
              <li>
                Press <kbd>⌘</kbd>+<kbd>⇧</kbd>+<kbd>7</kbd> anywhere, or click the menu-bar
                icon → <strong>Capture a region</strong>.
              </li>
              <li>Drag to select a region of the screen.</li>
              <li>The image is uploaded and its URL is copied to your clipboard.</li>
            </ol>
          </section>
          {!configured && (
            <p className="hint">Save your connection settings first.</p>
          )}

          <HistoryList items={history} onClear={handleClear} onNotify={notify} />
        </>
      )}

      {toast && <div className="toast">{toast}</div>}
    </main>
  );
}

export default App;
