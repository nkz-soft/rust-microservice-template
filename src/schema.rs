// @generated automatically by Diesel CLI.

diesel::table! {
    to_do_items (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 255]
        note -> Nullable<Varchar>,
    }
}