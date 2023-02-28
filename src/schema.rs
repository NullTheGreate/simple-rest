// @generated automatically by Diesel CLI.

diesel::table! {
    books (isbn_no) {
        isbn_no -> Varchar,
        name -> Varchar,
        author -> Varchar,
    }
}
