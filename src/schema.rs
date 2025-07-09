// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Integer,
        file_hash -> Text,
        file_name -> Text,
        file_path -> Text,
        size -> Integer,
        private -> Bool,
        created_at -> Timestamp,
    }
}
