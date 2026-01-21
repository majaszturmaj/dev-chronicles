-- Activity logs table with normalized fields for better querying and indexing
CREATE TABLE IF NOT EXISTS activity_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source TEXT NOT NULL,
    payload TEXT NOT NULL,
    timestamp DATETIME NOT NULL,
    is_processed BOOLEAN DEFAULT 0,
    
    -- Normalized fields extracted from payload
    log_type TEXT,                          -- 'command', 'browse', 'file_edit', etc.
    session_id TEXT,                        -- Groups logs into work sessions
    
    -- Terminal-specific fields
    command TEXT,
    exit_code INTEGER,
    duration_sec REAL,
    cwd TEXT,
    
    -- Browser-specific fields
    url TEXT,
    title TEXT,
    domain TEXT,
    time_on_page_sec INTEGER,
    
    -- VSCode-specific fields
    file_path TEXT,
    language TEXT,
    
    -- Common metadata
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Composite indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_unprocessed ON activity_logs(is_processed, timestamp);
CREATE INDEX IF NOT EXISTS idx_source_timestamp ON activity_logs(source, timestamp);
CREATE INDEX IF NOT EXISTS idx_session_id ON activity_logs(session_id, timestamp);
CREATE INDEX IF NOT EXISTS idx_domain ON activity_logs(domain, timestamp);

-- AI reports table with metadata linking to source logs
CREATE TABLE IF NOT EXISTS ai_reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    summary TEXT NOT NULL,
    generated_at DATETIME NOT NULL,
    
    -- Metadata about the report
    log_ids TEXT,                           -- JSON array of source log IDs
    log_count INTEGER DEFAULT 0,
    sources TEXT,                           -- Comma-separated: 'terminal,browser,vscode'
    time_range_start DATETIME,
    time_range_end DATETIME,
    session_id TEXT,                        -- If tied to a specific session
    
    -- Generation context
    model_used TEXT,
    temperature REAL DEFAULT 0.2
);

-- Indexes for efficient retrieval
CREATE INDEX IF NOT EXISTS idx_reports_generated ON ai_reports(generated_at DESC);
CREATE INDEX IF NOT EXISTS idx_reports_session ON ai_reports(session_id, generated_at DESC);

-- AI settings table (unchanged)
CREATE TABLE IF NOT EXISTS ai_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    provider_url TEXT NOT NULL,
    api_key TEXT,
    model_name TEXT NOT NULL DEFAULT 'gpt-4o-mini',
    
    -- New settings for better control
    temperature REAL DEFAULT 0.2,
    batch_size INTEGER DEFAULT 100,
    summary_frequency_min INTEGER DEFAULT 10,
    max_summary_tokens INTEGER DEFAULT 2000
);

-- Insert default settings only if table is empty
INSERT INTO ai_settings (id, provider_url, api_key, model_name, temperature, batch_size, summary_frequency_min, max_summary_tokens)
SELECT 1, 'http://localhost:1234/v1', NULL, 'gpt-4o-mini', 0.2, 100, 10, 2000
WHERE NOT EXISTS (SELECT 1 FROM ai_settings WHERE id = 1);
