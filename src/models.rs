use diesel::{
    prelude::*,
    sql_types::Date,
};

#[derive(Queryable)]
#[diesel(table_name = teste_table)]
pub struct Teste {
    pub id: i32,
}

#[derive(Queryable)]
#[diesel(table_name = students)]
pub struct Student {
    pub id: i32,
    pub email: String,
    pub birth_date: Date,
    pub password: String,
}

#[derive(Queryable)]
#[diesel(table_name = teachers)]
pub struct Teacher {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Queryable)]
#[diesel(table_name = questions)]
pub struct Question {
    pub id: i32,
    pub text: String,
}

#[derive(Queryable)]
#[diesel(table_name = classes)]
pub struct Class {
    pub id: i32,
    pub name: String,
    pub teacher_id: i32,
}

#[derive(Queryable)]
#[diesel(table_name = answers)]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub student_id: i32,
    pub text: String,
}

#[derive(Queryable)]
#[diesel(table_name = members)]
pub struct Member {
    pub id: i32,
    pub student_id: i32,
    pub class_id: i32,
}

#[derive(Queryable)]
#[diesel(table_name = results)]
pub struct Result {
    pub id: i32,

}

