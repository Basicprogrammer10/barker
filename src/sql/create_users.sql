CREATE TABLE IF NOT EXISTS users (
  id TEXT NOT NULL,
  username TEXT NOT NULL,
  password BLOB NOT NULL,
  ip TEXT NOT NULL,
  date INTEGER NOT NULL,
  UNIQUE(id, username)
)
