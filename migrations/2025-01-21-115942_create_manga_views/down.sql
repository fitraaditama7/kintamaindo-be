-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS manga_views CASCADE;
DROP MATERIALIZED VIEW IF EXISTS daily_popular_manga;

