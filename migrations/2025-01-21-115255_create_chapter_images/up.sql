-- Your SQL goes here
CREATE TABLE chapter_images (
    id SERIAL PRIMARY KEY,
    chapter_id INT NOT NULL,
    image_url TEXT NOT NULL,
    position INT NOT NULL,
    FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE
);