// @generated automatically by Diesel CLI.

diesel::table! {
    circle (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    letter (id) {
        id -> Int4,
        subject -> Varchar,
        content -> Varchar,
        circle_id -> Int4,
        writer_id -> Int4,
        reply_id -> Nullable<Int4>,
    }
}

diesel::table! {
    writer (id) {
        id -> Int4,
        title -> Varchar,
        name -> Varchar,
        pseudo -> Varchar,
    }
}

diesel::table! {
    writercircle (circle_id, writer_id) {
        circle_id -> Int4,
        writer_id -> Int4,
    }
}

diesel::joinable!(letter -> circle (circle_id));
diesel::joinable!(letter -> writer (writer_id));
diesel::joinable!(writercircle -> circle (circle_id));
diesel::joinable!(writercircle -> writer (writer_id));

diesel::allow_tables_to_appear_in_same_query!(
    circle,
    letter,
    writer,
    writercircle,
);
