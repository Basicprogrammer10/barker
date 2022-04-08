CREATE TABLE IF NOT EXISTS comments (
  id TEXT NOT NULL,
  bark_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  content TEXT NOT NULL,
  deleted BOOLEAN NOT NULL,
  date INTEGER NOT NULL,
  UNIQUE(id)
)
