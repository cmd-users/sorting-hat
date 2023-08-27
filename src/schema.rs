// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        question_id -> Int4,
        student_id -> Int4,
        text -> Text,
    }
}

diesel::table! {
    classes (id) {
        id -> Int4,
        name -> Text,
        teacher_id -> Int4,
    }
}

diesel::table! {
    example_table (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
    }
}

diesel::table! {
    members (id) {
        id -> Int4,
        student_id -> Int4,
        class_id -> Int4,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        text -> Text,
    }
}

diesel::table! {
    results (id) {
        id -> Int4,
    }
}

diesel::table! {
    students (id) {
        id -> Int4,
        email -> Text,
        birth_date -> Date,
        password -> Text,
    }
}

diesel::table! {
    teachers (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
    }
}

diesel::table! {
    teste_table (id) {
        id -> Int4,
    }
}

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(answers -> students (student_id));
diesel::joinable!(classes -> teachers (teacher_id));
diesel::joinable!(members -> classes (class_id));
diesel::joinable!(members -> students (student_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    classes,
    example_table,
    members,
    questions,
    results,
    students,
    teachers,
    teste_table,
);
