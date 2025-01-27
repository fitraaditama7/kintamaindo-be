-- This file should undo anything in `up.sql`
ALTER TABLE mangas DROP COLUMN IF EXISTS manga_type;
DROP TYPE IF EXISTS manga_type_enum;
