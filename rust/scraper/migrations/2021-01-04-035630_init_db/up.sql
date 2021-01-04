CREATE TABLE tropes (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE media_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE media (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    media_type_id INTEGER,
    FOREIGN KEY(media_type_id) REFERENCES media_types(id)
);

CREATE TABLE trope_entries (
    id INTEGER PRIMARY KEY,
    media_id INTEGER,
    trope_id INTEGER,
    FOREIGN KEY(media_id) REFERENCES media(id),
    FOREIGN KEY(trope_id) REFERENCES trope(id)
);