// @generated automatically by Diesel CLI.

diesel::table! {
    student (id) {
        id -> Integer,
        email -> Text,
        token -> Text,
        first_name -> Text,
        last_name -> Text,
    }
}

diesel::table! {
    teacher (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(student, teacher,);
