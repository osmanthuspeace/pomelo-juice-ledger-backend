// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Int4,
        date -> Date,
        kind -> Varchar,
        description -> Varchar,
        amount -> Float8,
        account -> Varchar,
    }
}
