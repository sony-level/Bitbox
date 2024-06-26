table! {
    use diesel::sql_types::*;
    use crate::models::*;

    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        email -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    classes (class_id) {
        class_id -> Int4,
        class_name -> Varchar,
        description -> Nullable<Text>,
        teacher_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    groups (group_id) {
        group_id -> Int4,
        group_name -> Varchar,
        class_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    group_members (group_member_id) {
        group_member_id -> Int4,
        group_id -> Int4,
        student_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    projects (project_id) {
        project_id -> Int4,
        project_name -> Varchar,
        description -> Nullable<Text>,
        class_id -> Int4,
        leader_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    evaluations (evaluation_id) {
        evaluation_id -> Int4,
        class_id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        max_score -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    evaluation_results (evaluation_result_id) {
        evaluation_result_id -> Int4,
        evaluation_id -> Int4,
        student_id -> Int4,
        score -> Int4,
        feedback -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    notifications (notification_id) {
        notification_id -> Int4,
        recipient_id -> Int4,
        sender_id -> Int4,
        message -> Text,
        read -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    reports (report_id) {
        report_id -> Int4,
        class_id -> Int4,
        report_title -> Varchar,
        report_content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    peer_evaluations (peer_evaluation_id) {
        peer_evaluation_id -> Int4,
        evaluation_id -> Int4,
        evaluator_id -> Int4,
        evaluatee_id -> Int4,
        score -> Int4,
        feedback -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(classes -> users (teacher_id));
joinable!(evaluation_results -> evaluations (evaluation_id));
joinable!(evaluation_results -> users (student_id));
joinable!(evaluations -> classes (class_id));
joinable!(group_members -> groups (group_id));
joinable!(group_members -> users (student_id));
joinable!(groups -> classes (class_id));
joinable!(notifications -> users (recipient_id));
joinable!(notifications -> users (sender_id));
joinable!(peer_evaluations -> evaluations (evaluation_id));
joinable!(peer_evaluations -> users (evaluatee_id));
joinable!(peer_evaluations -> users (evaluator_id));
joinable!(projects -> classes (class_id));
joinable!(projects -> users (leader_id));
joinable!(reports -> classes (class_id));

allow_tables_to_appear_in_same_query!(
    users,
    classes,
    groups,
    group_members,
    projects,
    evaluations,
    evaluation_results,
    notifications,
    reports,
    peer_evaluations,
);
