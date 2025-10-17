use crate::{middleware::*, repositories::*, services::*};
use ::axum::{
    Json,
    response::{IntoResponse, Response},
};
use ::indexmap::IndexMap;
use ::rand::prelude::SliceRandom;
use ::shared::{common::*, models::*, payloads::*, utils::*};
use ::std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

pub async fn get_quiz_record(session: &Session, id: impl Into<String>) -> Result<Response> {
    let quiz_rec_arc = Store::find::<QuizRecord>(&session.workspace, id).await?;
    let quiz_rec = { quiz_rec_arc.read().await.clone() };
    Ok(Json(quiz_rec).into_response())
}

pub async fn get_quiz_record_base(session: &Session, id: impl Into<String>) -> Result<Response> {
    let quiz_rec_arc = Store::find::<QuizRecord>(&session.workspace, id).await?;
    let quiz_rec_base = {
        let quiz_rec_guard = quiz_rec_arc.read().await;
        quiz_rec_guard.to_base()
    };
    Ok(Json(quiz_rec_base).into_response())
}

pub async fn create_quiz_record(session: &Session, payload: CreateTaskPayload) -> Result<Task> {
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;
    let CreateTaskPayload {
        id,
        node,
        name,
        path,
        categories,
    } = payload;

    let nodes = {
        let ws_guard = ws_arc.read().await;
        ws_guard.unit_tree.node_descendants(&node)
    };

    let students_fut = StudentRepository::list_by_filter(&session.workspace, Some(nodes));
    let quiz_fut = Store::find::<Quiz>(&session.workspace, id);
    let (mut students_vec, quiz_arc) = tokio::try_join!(students_fut, quiz_fut)?;

    if students_vec.is_empty() {
        Err((StatusCode::BAD_REQUEST, "no-students-found"))?
    }

    students_vec.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    let mut students: IndexMap<String, QuizRecordStudent> =
        IndexMap::with_capacity(students_vec.len());
    for s in students_vec {
        let sid = s.id.clone();
        students.insert(
            sid,
            QuizRecordStudent {
                id: s.id,
                rank: s.rank,
                name: s.name,
                attempts: 0,
                grade: 0,
            },
        );
    }

    let record = {
        let quiz_guard = quiz_arc.read().await;

        let mut task_categories =
            IndexMap::<String, QuizRecordCategory>::with_capacity(categories.len());
        let mut total_count = 0_i64;
        let max_count = quiz_guard.categories.len();

        for category_req in categories {
            let Some(category) = quiz_guard.categories.get(&category_req.id) else {
                continue;
            };
            let count = category_req.count.min(max_count);
            if count == 0 {
                continue;
            }
            total_count += count as i64;

            let category_id = category.id.clone();
            task_categories.insert(
                category_id.clone(),
                QuizRecordCategory {
                    id: category_id,
                    name: category.name.clone(),
                    count,
                },
            );
        }

        let answers = Grid::<HashMap<String, HashSet<String>>>::new(
            students.len(),
            task_categories.len(),
            Default::default(),
        );

        let results = Grid::<usize>::new(students.len(), task_categories.len(), 0_usize);

        QuizRecord {
            id: safe_nanoid!(),
            workspace: session.workspace.clone(),
            quiz: quiz_guard.id.clone(),
            name,
            node,
            path,
            attempts: quiz_guard.attempts,
            duration: quiz_guard.duration * total_count,
            grade: quiz_guard.grade.clone(),
            categories: task_categories,
            answers,
            students,
            results,
            metadata: Metadata::new(&session.username),
        }
    };

    let task = Task {
        id: record.id.clone(),
        workspace: record.workspace.clone(),
        kind: EntityKind::QuizRecord,
        name: record.name.clone(),
        node: record.node.clone(),
        path: record.path.clone(),
        progress: 0,
        metadata: record.metadata.clone(),
    };

    Store::upsert(record).await?;
    Ok(task)
}

pub async fn get_quiz_categories(
    session: &Session,
    id: impl Into<String>,
) -> Result<Vec<TaskCategory>> {
    let quiz_arc = Store::find::<Quiz>(&session.workspace, id).await?;

    let categories = {
        let quiz_guard = quiz_arc.read().await;

        quiz_guard
            .categories
            .values()
            .map(|c| TaskCategory {
                id: c.id.clone(),
                name: c.name.clone(),
                count: c.count,
                total: c.questions.len(),
                checked: c.count > 0,
            })
            .collect::<Vec<_>>()
    };

    Ok(categories)
}

pub async fn get_quiz_activity_details(
    workspace: impl Into<String>,
    task_id: impl Into<String>,
    student: impl Into<String>,
) -> Result<Response> {
    let ws_id = workspace.into();
    let task_id = task_id.into();
    let student_id = student.into();

    let quiz_rec_arc = Store::find::<QuizRecord>(&ws_id, task_id).await?;
    let activity = {
        let quiz_rec_guard = quiz_rec_arc.read().await;
        let student_idx = quiz_rec_guard
            .students
            .get_index_of(&student_id)
            .ok_or((StatusCode::NOT_FOUND, "student-not-found"))?;
        let scores = quiz_rec_guard.results.get_row(student_idx);
        let score = if scores.is_empty() {
            0
        } else {
            let sum: i32 = scores.iter().map(|&&v| v as i32).sum();
            ((sum as f64) / (scores.len() as f64)).round() as usize
        };
        let student = quiz_rec_guard.students.index(student_idx);
        let can_take =
            quiz_rec_guard.attempts == 0 || quiz_rec_guard.attempts > student.attempts;

        QuizActivityDetails {
            workspace: quiz_rec_guard.workspace.clone(),
            quiz: quiz_rec_guard.quiz.clone(),
            quiz_name: quiz_rec_guard.name.clone(),
            duration: quiz_rec_guard.duration,
            student: student.id.clone(),
            student_rank: student.rank.clone(),
            student_name: student.name.clone(),
            grade: student.grade,
            score,
            can_take,
        }
    };

    Ok(Json(activity).into_response())
}

pub async fn get_quiz_activity(
    workspace: impl Into<String>,
    task_id: impl Into<String>,
    student: impl Into<String>,
) -> Result<Response> {
    let ws_id = workspace.into();
    let task_id = task_id.into();
    let student_id = student.into();

    let quiz_rec_arc = Store::find::<QuizRecord>(&ws_id, &task_id).await?;
    let (categories_map, quiz_id, duration) = {
        let quiz_rec_guard = quiz_rec_arc.read().await;
        if quiz_rec_guard
            .students
            .get(&student_id)
            .ok_or((StatusCode::NOT_FOUND, "student-not-found"))?
            .attempts
            > quiz_rec_guard.attempts && quiz_rec_guard.attempts > 0
        {
            Err("attempts-exceeded")?
        }
        let map = quiz_rec_guard
            .categories
            .values()
            .map(|c| (c.id.clone(), c.count))
            .collect::<HashMap<String, usize>>();

        (
            map,
            quiz_rec_guard.quiz.clone(),
            quiz_rec_guard.duration,
        )
    };

    let quiz = Store::find::<Quiz>(&ws_id, &quiz_id)
        .await?
        .read()
        .await
        .clone();

    let mut questions = Vec::new();
    for (category_id, category_count) in categories_map {
        let question = generate_category_questions(&quiz, &category_id, category_count).await;
        questions.extend(question);
    }
    questions.shuffle(&mut rand::rng());

    let activity = QuizActivity {
        workspace: ws_id,
        task: task_id,
        quiz: quiz_id,
        duration,
        student: student_id,
        questions: questions
            .into_iter()
            .map(|q| (q.id.clone(), q))
            .collect::<IndexMap<String, QuizActivityQuestion>>(),
    };

    Ok(Json(activity).into_response())
}

pub async fn update_quiz_activity(activity: QuizActivity) -> Result<()> {
    let quiz_rec_arc = Store::find::<QuizRecord>(&activity.workspace, &activity.task).await?;
    let (quiz_id, categories, student, student_idx) = {
        let quiz_rec_guard = quiz_rec_arc.read().await;
        let student_idx = quiz_rec_guard
            .students
            .get_index_of(&activity.student)
            .ok_or((StatusCode::NOT_FOUND, "student-not-found"))?;
        let student = quiz_rec_guard.students.index(student_idx).clone();
        if quiz_rec_guard.attempts > 0 && student.attempts >= quiz_rec_guard.attempts {
            Err("attempts-exceeded")?
        }
        let quiz_id = quiz_rec_guard.quiz.clone();
        let categories = quiz_rec_guard.categories.clone();
        (quiz_id, categories, student, student_idx)
    };
    let quiz = Store::find::<Quiz>(&activity.workspace, &quiz_id)
        .await?
        .read()
        .await
        .clone();

    let (total_grade, result, fail_important, answers) = categories.iter().fold(
        (0, Vec::with_capacity(categories.len()), false, Vec::<HashMap<String, HashSet<String>>>::new()),
        |(mut total_grade, mut result, mut fail, mut answers), (category_id, category)| {
            let activity_questions = activity
                .questions
                .iter()
                .filter(|(_, q)| &q.category == category_id)
                .collect::<Vec<_>>();
            let (correct_questions, student_answers) =
                activity_questions
                    .iter()
                    .fold((0, HashMap::<String, HashSet<String>>::new()), |(mut correct, mut answers), (question_id, question)| {
                        let Some(quiz_question) = quiz
                            .categories
                            .get(&category.id)
                            .and_then(|c| c.questions.get(&question.id))
                        else {
                            return (correct, answers);
                        };
                        let correct_answers = quiz_question
                            .answers
                            .values()
                            .filter(|a| a.correct)
                            .map(|a| a.id.clone())
                            .collect::<HashSet<String>>();
                        if question.answered == correct_answers {
                            correct += 1;
                        }
                        answers.insert(quiz_question.id.clone(), question.answered.clone());
                        (correct, answers)
                    });

            let score = if !activity_questions.is_empty() {
                ((correct_questions as f64) / (activity_questions.len() as f64) * 100.0) as usize
            } else {
                0
            };
            let grade = quiz.grade.calc(score);
            let important = quiz
                .categories
                .get(category_id)
                .and_then(|c| Some(c.important))
                .unwrap_or(false);
            if important && grade < 3 {
                fail = true;
            }
            result.push(score);
            total_grade += grade;
            answers.push(student_answers);

            (total_grade, result, fail, answers)
        },
    );

    let categories_count = if !categories.is_empty() { categories.len() } else { 1 };
    let grade = if !fail_important { ((total_grade as f64) / (categories_count as f64) + 0.5).floor() as usize } else { 2 };

    if student.grade > grade { return Ok(()) }

    let (snapshot, progress) = {
        let mut quiz_rec_guard = quiz_rec_arc.write().await;
        if let Some(student) = quiz_rec_guard.students.get_mut(&student.id) {
            student.attempts += 1;
            student.grade = grade;
        }
        quiz_rec_guard.answers.set_row(student_idx, answers);
        quiz_rec_guard.results.set_row(student_idx, result);

        let count = quiz_rec_guard.students.values().filter(|s| s.grade > 0).count();
        let progress = if quiz_rec_guard.students.is_empty() { 0 } else { (count * 100) / quiz_rec_guard.students.len() };

        (quiz_rec_guard.clone(), progress)
    };
    Store::upsert(snapshot).await?;

    let tasks_arc = Store::find::<Tasks>(activity.workspace, TASKS).await?;
    let snapshot = {
        let mut tasks_guard = tasks_arc.write().await;
        let task = tasks_guard
            .get_mut(&activity.task)
            .ok_or((StatusCode::NOT_FOUND, "task-not-found"))?;
        task.progress = progress;
        tasks_guard.clone()
    };
    Store::upsert(snapshot).await?;

    Ok(())
}

async fn generate_category_questions(
    quiz: &Quiz,
    id: &str,
    count: usize,
) -> Vec<QuizActivityQuestion> {
    let mut ids = quiz
        .categories
        .get(id)
        .map(|c| c.questions.keys().collect::<Vec<_>>())
        .unwrap_or_default();
    ids.shuffle(&mut rand::rng());

    let count = count.min(ids.len());
    let selected_questions = ids.iter().take(count);

    let mut questions = Vec::new();
    for &q in selected_questions {
        if let Some(question) = generate_question(quiz, id, q).await {
            questions.push(question)
        }
    }

    questions
}

async fn generate_question(
    quiz: &Quiz,
    category_id: &str,
    question_id: &str,
) -> Option<QuizActivityQuestion> {
    let mut answers = quiz
        .categories
        .get(category_id)
        .and_then(|c| c.questions.get(question_id))
        .and_then(|q| Some(q.answers.values().collect::<Vec<_>>()))
        .unwrap_or_default();
    answers.shuffle(&mut rand::rng());

    let kind = if answers.iter().filter(|a| a.correct).count() == 1 {
        QuizActivityQuestionKind::Single
    } else {
        QuizActivityQuestionKind::Multiple
    };

    let answers = answers
        .into_iter()
        .cloned()
        .map(|a| {
            (
                a.id.clone(),
                QuizActivityAnswer {
                    id: a.id,
                    name: a.name,
                    img: a.img,
                },
            )
        })
        .collect::<IndexMap<String, QuizActivityAnswer>>();

    quiz.categories
        .get(category_id)
        .and_then(|c| c.questions.get(question_id))
        .and_then(|q| {
            Some(QuizActivityQuestion {
                id: q.id.clone(),
                category: category_id.to_string(),
                kind,
                name: q.name.clone(),
                img: q.img,
                answers,
                answered: Default::default(),
            })
        })
}
