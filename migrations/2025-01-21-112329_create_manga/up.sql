-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS mangas (
   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   title VARCHAR(255) NOT NULL,
   title_english VARCHAR(255),
   title_alternative VARCHAR(255),
   synonyms TEXT,
   status VARCHAR(50),
   chapters INT,
   volumes INT,
   score DECIMAL(3, 2),
   serialization TEXT,
   synopsis TEXT,
   image_url TEXT,
   image_background_url TEXT,
   mal_url TEXT,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
   deleted_at TIMESTAMP
);
