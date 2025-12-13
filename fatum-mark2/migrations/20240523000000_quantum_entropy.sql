CREATE TABLE IF NOT EXISTS quantum_entropy_batches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'collecting', -- 'collecting', 'completed'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS quantum_entropy_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    batch_id INTEGER NOT NULL,
    pulse_round INTEGER,
    hex_value TEXT NOT NULL, -- Stored as hex string or base64
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(batch_id) REFERENCES quantum_entropy_batches(id) ON DELETE CASCADE
);
