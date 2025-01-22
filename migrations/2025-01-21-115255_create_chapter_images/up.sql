-- Your SQL goes here
CREATE TABLE chapter_images (
    id UUID PRIMARY KEY,
    chapter_id UUID NOT NULL,
    image_url TEXT NOT NULL,
    position INT NOT NULL,
    FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE
);