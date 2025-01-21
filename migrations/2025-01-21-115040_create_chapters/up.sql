-- Your SQL goes here
CREATE TABLE chapters (
    id SERIAL PRIMARY KEY,
    manga_id INT NOT NULL,
    chapter_number INT NOT NULL,
    sub_chapter_number INT DEFAULT NULL,
    title VARCHAR(255) NOT NULL,
    release_date DATE,
    FOREIGN KEY (manga_id) REFERENCES manga(id) ON DELETE CASCADE
);