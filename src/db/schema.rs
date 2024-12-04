// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Int4,
        date -> Date,
        kind -> Varchar,
        description -> Text,
        amount -> Float8,
        account -> Varchar,
    }
}
