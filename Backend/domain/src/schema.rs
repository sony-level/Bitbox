// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    auth_tokens (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        #[max_length = 255]
        token -> Varchar,
        created_at -> Nullable<Timestamp>,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    class_users (class_id, user_id) {
        class_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    classes (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        start_date -> Date,
        end_date -> Date,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    email_confirmations (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        token -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Nullable<Timestamp>,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    evaluation_results (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        group_id -> Nullable<Uuid>,
        average_score -> Float8,
        final_score -> Float8,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    evaluations (id) {
        id -> Uuid,
        evaluator_id -> Nullable<Uuid>,
        evaluatee_id -> Nullable<Uuid>,
        group_id -> Nullable<Uuid>,
        project_id -> Nullable<Uuid>,
        score -> Int4,
        comments -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    group_users (group_id, user_id) {
        group_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    groups (id) {
        id -> Uuid,
        #[max_length = 100]
        group_name -> Varchar,
        project_id -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    notifications (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        message -> Text,
        sent_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    projects (id) {
        id -> Uuid,
        #[max_length = 100]
        project_name -> Varchar,
        class_id -> Nullable<Uuid>,
        descriptions -> Nullable<Text>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    registration_links (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        #[max_length = 255]
        token -> Varchar,
        expires_at -> Timestamp,
        used_at -> Nullable<Timestamp>,
        used_by -> Nullable<Uuid>,
        project_id -> Nullable<Uuid>,
        group_id -> Nullable<Uuid>,
        class_id -> Nullable<Uuid>,
        first_name -> Varchar,
        last_name -> Varchar,
        role -> UserRole,
        #[max_length = 255]
        email -> Varchar,
        link -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        #[max_length = 255]
        totp_secret -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        jwt_secret -> Varchar,
        role -> UserRole,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(auth_tokens -> users (user_id));
diesel::joinable!(class_users -> classes (class_id));
diesel::joinable!(class_users -> users (user_id));
diesel::joinable!(evaluation_results -> projects (group_id));
diesel::joinable!(evaluation_results -> users (user_id));
diesel::joinable!(evaluations -> groups (group_id));
diesel::joinable!(evaluations -> projects (project_id));
diesel::joinable!(group_users -> groups (group_id));
diesel::joinable!(group_users -> users (user_id));
diesel::joinable!(groups -> projects (project_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(projects -> classes (class_id));
diesel::joinable!(registration_links -> classes (class_id));
diesel::joinable!(registration_links -> groups (group_id));
diesel::joinable!(registration_links -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_tokens,
    class_users,
    classes,
    email_confirmations,
    evaluation_results,
    evaluations,
    group_users,
    groups,
    notifications,
    projects,
    registration_links,
    users,
);
