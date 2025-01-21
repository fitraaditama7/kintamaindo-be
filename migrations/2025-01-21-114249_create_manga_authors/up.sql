-- Your SQL goes here
CREATE TABLE IF NOT EXISTS manga_authors (
    manga_id UUID NOT NULL,
    author_id UUID NOT NULL,
    author_type VARCHAR(50) NOT NULL CHECK (author_type IN ('story', 'art')),
    PRIMARY KEY (manga_id, author_id),
    FOREIGN KEY (manga_id) REFERENCES manga (id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES authors (id) ON DELETE CASCADE
);