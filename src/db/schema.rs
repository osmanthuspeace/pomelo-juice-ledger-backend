// @generated automatically by Diesel CLI.

diesel::table! {
    summary (id) {
        balance -> Nullable<Float8>,
        storage -> Nullable<Float8>,
        alipay -> Nullable<Float8>,
        wechat -> Nullable<Float8>,
        bankofchina -> Nullable<Float8>,
        icbc -> Nullable<Float8>,
        id -> Int4,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    summary,
    transactions,
);
