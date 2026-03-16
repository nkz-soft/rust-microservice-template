use diesel::table;

// @generated automatically by Diesel CLI.
table! {
    to_do_items (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 255]
        note -> Nullable<Varchar>,
        #[max_length = 32]
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        due_at -> Nullable<Timestamptz>,
        version -> Int4,
    }
}
