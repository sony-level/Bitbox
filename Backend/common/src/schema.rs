// @generated automatically by Diesel CLI.

diesel::table! {
    classes (class_id) {
        class_id -> Int4,
        #[max_length = 100]
        class_name -> Varchar,
        description -> Nullable<Text>,
        teacher_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    reports (report_id) {
        report_id -> Int4,
        class_id -> Int4,
        #[max_length = 100]
        report_title -> Varchar,
        report_content -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 20]
        role -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(classes -> users (teacher_id));
diesel::joinable!(reports -> classes (class_id));

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    reports,
    users,
);
