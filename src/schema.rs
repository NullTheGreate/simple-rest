// @generated automatically by Diesel CLI.

diesel::table! {
    books (isbn_no) {
        isbn_no -> Varchar,
        name -> Varchar,
        author -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        user_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(books, users,);
