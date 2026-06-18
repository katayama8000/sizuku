import { openUrl } from "@tauri-apps/plugin-opener";
import { copyToClipboard } from "../lib/ipc";
import type { HistoryItem } from "../types";

type Props = {
  items: HistoryItem[];
  onClear: () => void;
  onNotify: (message: string) => void;
};

function formatDate(millis: number): string {
  const d = new Date(millis);
  return d.toLocaleString();
}

export function HistoryList({ items, onClear, onNotify }: Props) {
  const handleCopy = async (url: string) => {
    try {
      await copyToClipboard(url);
      onNotify("Copied URL to clipboard");
    } catch (e) {
      onNotify(`Copy failed: ${String(e)}`);
    }
  };

  return (
    <section className="history">
      <div className="history-head">
        <h2>Upload history</h2>
        {items.length > 0 && (
          <button type="button" className="link" onClick={onClear}>
            Clear history
          </button>
        )}
      </div>
      {items.length === 0 ? (
        <p className="empty">No history yet.</p>
      ) : (
        <ul>
          {items.map((item) => (
            <li key={`${item.key}-${item.created_at}`}>
              <img src={item.url} alt={item.key} loading="lazy" />
              <div className="meta">
                <span className="url" title={item.url}>
                  {item.url}
                </span>
                <span className="time">{formatDate(item.created_at)}</span>
              </div>
              <div className="actions">
                <button type="button" onClick={() => handleCopy(item.url)}>
                  Copy
                </button>
                <button type="button" onClick={() => openUrl(item.url)}>
                  Open
                </button>
              </div>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
