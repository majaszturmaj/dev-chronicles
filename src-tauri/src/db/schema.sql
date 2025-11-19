-- Activity logs table
CREATE TABLE IF NOT EXISTS activity_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source TEXT NOT NULL,
    payload TEXT NOT NULL,
    timestamp DATETIME NOT NULL,
    is_processed BOOLEAN DEFAULT 0
);

-- Create index if it doesn't exist
CREATE INDEX IF NOT EXISTS idx_unprocessed ON activity_logs(is_processed, timestamp);

-- AI reports table
CREATE TABLE IF NOT EXISTS ai_reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    summary TEXT NOT NULL,
    generated_at DATETIME NOT NULL
);

-- AI settings table
CREATE TABLE IF NOT EXISTS ai_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    provider_url TEXT NOT NULL,
    api_key TEXT,
    model_name TEXT NOT NULL DEFAULT 'gpt-4o-mini'
);

-- Insert default settings only if table is empty
INSERT INTO ai_settings (id, provider_url, api_key, model_name)
SELECT 1, 'http://localhost:1234/v1', NULL, 'gpt-4o-mini'
WHERE NOT EXISTS (SELECT 1 FROM ai_settings WHERE id = 1);
