CREATE TABLE IF NOT EXISTS profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    birth_year INTEGER,
    birth_month INTEGER,
    birth_day INTEGER,
    birth_hour INTEGER,
    gender TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id INTEGER,
    tool_type TEXT NOT NULL, -- 'fengshui', 'divination', 'decision'
    summary TEXT, -- Short JSON or text summary
    full_report JSON, -- The full JSON output
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(profile_id) REFERENCES profiles(id)
);
