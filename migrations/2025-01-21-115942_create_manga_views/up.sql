-- Your SQL goes here
CREATE TABLE IF NOT EXISTS manga_views (
     id UUID PRIMARY KEY,
     manga_id UUID NOT NULL,
     viewed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
     FOREIGN KEY (manga_id) REFERENCES mangas(id) ON DELETE CASCADE
);

CREATE INDEX idx_manga_views_viewed_at ON manga_views (viewed_at);
CREATE INDEX idx_manga_views_manga_id ON manga_views (manga_id);

-- TODO: need to add refresh materialized view
CREATE MATERIALIZED VIEW IF NOT EXISTS daily_popular_manga AS
SELECT manga_id, DATE(viewed_at) AS view_date, COUNT(*) AS view_count
FROM manga_views
GROUP BY manga_id, DATE(viewed_at);
