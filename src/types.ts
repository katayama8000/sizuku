export type Settings = {
  base_url: string;
  user: string;
  pass: string;
};

export type HistoryItem = {
  key: string;
  url: string;
  /** UNIX epoch milliseconds */
  created_at: number;
};
