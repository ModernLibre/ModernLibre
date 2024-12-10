// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        author -> Nullable<Varchar>,
        description -> Nullable<Text>,
        status -> Nullable<Int4>,
        rating -> Nullable<Float8>,
        added_date -> Date,
        #[max_length = 255]
        cover_url -> Varchar,
        #[max_length = 255]
        extension -> Varchar,
    }
}
