-- Your SQL goes here
CREATE TABLE chapters (
    id UUID PRIMARY KEY,
    manga_id UUID NOT NULL,
    chapter_number INT NOT NULL,
    sub_chapter_number INT DEFAULT NULL,
    title VARCHAR(255) NOT NULL,
    release_date DATE,
    FOREIGN KEY (manga_id) REFERENCES mangas(id) ON DELETE CASCADE
);
