// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Text,
        payload -> Nullable<Jsonb>,
        created_at -> Timestamp,
    }
}
