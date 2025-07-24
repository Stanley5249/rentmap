CREATE TABLE rent_list (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    url TEXT NOT NULL,
    page_count INTEGER,
    item_count INTEGER,
    UNIQUE (url, created_at)
);

CREATE TABLE rent_item_summary (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    list_id INTEGER NOT NULL,
    url TEXT NOT NULL,
    title TEXT,
    price TEXT,
    tags TEXT,
    txts TEXT,
    images TEXT,
    FOREIGN KEY (list_id) REFERENCES rent_list (id) ON DELETE CASCADE
);

CREATE TABLE rent_item (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    url TEXT NOT NULL UNIQUE,
    title TEXT,
    labels TEXT,
    patterns TEXT,
    content TEXT,
    phone TEXT,
    album TEXT,
    area TEXT,
    floor TEXT,
    price TEXT,
    address TEXT
);

CREATE TABLE page_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    html TEXT NOT NULL
);

CREATE INDEX idx_rent_list_url_latest ON rent_list (url, created_at DESC);

CREATE INDEX idx_rent_item_summary_list ON rent_item_summary (list_id);

CREATE INDEX idx_rent_item_summary_url ON rent_item_summary (url);

CREATE INDEX idx_rent_item_url ON rent_item (url);

CREATE INDEX idx_page_cache_url ON page_cache (url);