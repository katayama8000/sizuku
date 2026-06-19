export type Settings = {
  base_url: string;
  user: string;
  pass: string;
  /** Run as a menu-bar resident app (no Dock icon / no window on launch). */
  menu_bar_mode: boolean;
};

export type HistoryItem = {
  key: string;
  url: string;
  /** UNIX epoch milliseconds */
  created_at: number;
};
