-- Your SQL goes here
CREATE TABLE IF NOT EXISTS genres (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

INSERT INTO genres (name) VALUES
  ('Action'),
  ('Adventure'),
  ('Avant Garde'),
  ('Award Winning'),
  ('Boys Love'),
  ('Comedy'),
  ('Drama'),
  ('Fantasy'),
  ('Girls Love'),
  ('Gourmet'),
  ('Horror'),
  ('Mystery'),
  ('Romance'),
  ('Sci-Fi'),
  ('Slice of Life'),
  ('Sports'),
  ('Supernatural'),
  ('Suspense'),
  ('Ecchi'),
  ('Adult Cast'),
  ('Anthropomorphic'),
  ('CGDCT'),
  ('Childcare'),
  ('Combat Sports'),
  ('Crossdressing'),
  ('Delinquents'),
  ('Detective'),
  ('Educational'),
  ('Gag Humor'),
  ('Gore'),
  ('Harem'),
  ('High Stakes Game'),
  ('Historical'),
  ('Idols'),
  ('Isekai'),
  ('Iyashikei'),
  ('Love Polygon'),
  ('Love Status Quo'),
  ('Mahou Shoujo'),
  ('Martial Arts'),
  ('Mecha'),
  ('Medical'),
  ('Memoir'),
  ('Military'),
  ('Music'),
  ('Mythology'),
  ('Organized Crime'),
  ('Otaku Culture'),
  ('Parody'),
  ('Performing Arts'),
  ('Pets'),
  ('Psychological'),
  ('Racing'),
  ('Reincarnation'),
  ('Reverse Harem'),
  ('Samurai'),
  ('School'),
  ('Showbiz'),
  ('Space'),
  ('Strategy Game'),
  ('Super Power'),
  ('Survival'),
  ('Team Sports'),
  ('Time Travel'),
  ('Urban Fantasy'),
  ('Vampire'),
  ('Video Game'),
  ('Villainess'),
  ('Visual Arts'),
  ('Workplace'),
  ('Josei'),
  ('Kids'),
  ('Seinen'),
  ('Shoujo'),
  ('Shounen');