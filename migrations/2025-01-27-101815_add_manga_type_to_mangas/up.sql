-- Your SQL goes here
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'manga_type_enum') THEN
CREATE TYPE manga_type_enum AS ENUM ('Manga', 'Manhwa', 'Manhua');
END IF;
END $$;

ALTER TABLE mangas
    ADD COLUMN manga_type manga_type_enum NOT NULL DEFAULT 'Manga';
