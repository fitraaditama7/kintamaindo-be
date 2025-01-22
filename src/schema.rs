// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    chapter_images (id) {
        id -> Uuid,
        chapter_id -> Uuid,
        image_url -> Text,
        position -> Int4,
    }
}

diesel::table! {
    chapters (id) {
        id -> Uuid,
        manga_id -> Uuid,
        chapter_number -> Int4,
        sub_chapter_number -> Nullable<Int4>,
        #[max_length = 255]
        title -> Varchar,
        release_date -> Nullable<Date>,
    }
}

diesel::table! {
    genres (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    manga_authors (manga_id, author_id) {
        manga_id -> Uuid,
        author_id -> Uuid,
        #[max_length = 50]
        author_type -> Varchar,
    }
}

diesel::table! {
    manga_genres (manga_id, genre_id) {
        manga_id -> Uuid,
        genre_id -> Uuid,
    }
}

diesel::table! {
    manga_views (id) {
        id -> Uuid,
        manga_id -> Uuid,
        viewed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    mangas (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        title_english -> Nullable<Varchar>,
        #[max_length = 255]
        title_alternative -> Nullable<Varchar>,
        synonyms -> Nullable<Text>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        chapters -> Nullable<Int4>,
        volumes -> Nullable<Int4>,
        score -> Nullable<Numeric>,
        serialization -> Nullable<Text>,
        synopsis -> Nullable<Text>,
        image_url -> Nullable<Text>,
        image_background_url -> Nullable<Text>,
        mal_url -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(chapter_images -> chapters (chapter_id));
diesel::joinable!(chapters -> mangas (manga_id));
diesel::joinable!(manga_authors -> authors (author_id));
diesel::joinable!(manga_authors -> mangas (manga_id));
diesel::joinable!(manga_genres -> genres (genre_id));
diesel::joinable!(manga_genres -> mangas (manga_id));
diesel::joinable!(manga_views -> mangas (manga_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    chapter_images,
    chapters,
    genres,
    manga_authors,
    manga_genres,
    manga_views,
    mangas,
);
