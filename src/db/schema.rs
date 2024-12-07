// @generated automatically by Diesel CLI.

diesel::table! {
    summary (id) {
        id -> Int4,
        account -> Varchar,
        amount -> Float8,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        date -> Date,
        kind -> Varchar,
        description -> Text,
        amount -> Float8,
        account -> Varchar,
        balance -> Float8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    summary,
    transactions,
);
