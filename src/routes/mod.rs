use diesel::{
    query_dsl::RunQueryDsl,
    QueryDsl,
};
use rocket::serde::json::Json;
use serde::Serialize;

use crate::establish_connection;

#[derive(Serialize)]
pub struct QuizNextResponse {
    current: usize,
    question: String,
    question_count: usize,
}

#[get("/quiz/next")]
pub fn quiz_next() -> Json<QuizNextResponse> {
    use super::schema::answers::dsl::*;
    use super::schema::questions::dsl::*;

    let mut conn = establish_connection();

    let user_answer_count = answers
        .count()
        .get_result::<i64>(&mut conn)
        .expect("Error fetching answer count of user TEST at route `/quiz/next`")
        as usize
    ;

    let question: String = questions
        .order(crate::questions::id)
        .select(crate::questions::text)
        .first(&mut conn)
        .expect("Error fetching question text at route `/quiz/next`")
    ;

    let question_count: usize = questions
        .count()
        .get_result::<i64>(&mut conn)
        .expect("Error fetching question count at route `/quiz/next`")
        as usize
    ;

    Json(QuizNextResponse {
        current: user_answer_count,
        question,
        question_count,
    })
}
