use crate::prelude::*;
use ::std::collections::{HashMap, HashSet};

#[derive(Default, Clone, PartialEq)]
struct QuizReportState {
    pub is_supervisor: bool,
    pub as_percentage: bool,
    pub students_details: bool,
    pub stats: bool,
    pub active_student: Option<String>,
}

#[component]
pub fn QuizReport(entity: ReadSignal<String>) -> Element {
    let is_supervisor = use_context::<Arc<Claims>>().is_supervisor();
    let mut quiz = use_context_provider(|| Signal::new(Quiz::default()));
    let mut quiz_rec = use_context_provider(|| Signal::new(QuizRecord::default()));
    let quiz_rec_guard = quiz_rec.read();
    let mut state = use_context_provider(|| {
        Signal::new(QuizReportState {
            is_supervisor,
            stats: true,
            ..Default::default()
        })
    });

    use_effect(move || {
        api_fetch!(
            GET,
            format!(
                "/api/v1/entities/payload/{kind}/{id}",
                kind = EntityKind::QuizRecord,
                id = entity.read()
            ),
            on_success = move |body: QuizRecord| {
                if is_supervisor {
                    api_fetch!(
                        GET,
                        format!("/api/v1/manager/quizzes/{id}", id = body.quiz),
                        on_success = move |body: Quiz| quiz.set(body)
                    )
                }
                quiz_rec.set(body);
            }
        );
    });

    if quiz_rec_guard.id.is_empty() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "flex shrink-0 w-full min-h-0 print:hidden p-1",
            ul {
                class: "menu menu-horizontal p-0 m-0 text-base-content flex-nowrap",
                li {
                    button {
                        class: "hover:text-info",
                        onclick: move |event: MouseEvent| {
                            event.prevent_default();
                            event.stop_propagation();
                            document::eval("window.print()");
                        },
                        i { class: "bi bi-printer" }
                        { t!("print") }
                    }
                }
                div { class: "divider divider-horizontal m-1 w-1" }
                div {
                    class: "tooltip tooltip-bottom",
                    "data-tip": t!("absent-uncertified-students"),
                    li {
                        button {
                            class: if state.read().students_details { "bg-secondary/30 text-secondary" } else { "" },
                            onclick: move |_| state.with_mut(|s| s.students_details = !s.students_details),
                            i { class: "bi bi-person-lines-fill" }
                        }
                    }
                }
                div { class: "m-0 w-1" }
                div {
                    class: "tooltip tooltip-bottom",
                    "data-tip": t!("stats"),
                    li {
                        button {
                            class: if state.read().stats { "bg-secondary/30 text-secondary" } else { "" },
                            onclick: move |_| state.with_mut(|s| s.stats = !s.stats),
                            i { class: "bi bi-graph-up-arrow" }
                        }
                    }
                }
                div { class: "divider divider-horizontal m-1 w-1" }
                div {
                    class: "tooltip tooltip-bottom",
                    "data-tip": t!("grade-or-percentage"),
                    li {
                        label {
                            class: "swap swap-rotate text-sm",
                            input {
                                r#type: "checkbox",
                                onchange: move |evt| state.with_mut(|s| s.as_percentage = evt.checked())
                            }
                            i { class: "bi bi-percent swap-on" }
                            i { class: "bi bi-star-half swap-off" }
                        }
                    }
                }
                if state.read().active_student.is_some() {
                    div { class: "divider divider-horizontal m-1 w-1" }
                    li {
                        button {
                            onclick: move |_| state.with_mut(|s| s.active_student = None),
                            i { class: "bi bi-file-earmark-text" }
                            { t!("report") }
                        }
                    }
                }
            }
        }
        div {
            class: "flex flex-1 flex-col print-area overflow-auto px-5 print:px-1",
            "data-theme": "lofi",
            if state.read().active_student.is_some() {
                RenderStudentReport {}
            } else {
                RenderQuizReport {}
            }
        }
    }
}

#[component]
fn RenderQuizReport() -> Element {
    let mut state = use_context::<Signal<QuizReportState>>();
    let quiz_rec = use_context::<Signal<QuizRecord>>();
    let quiz_rec_guard = quiz_rec.read();

    let has_ranks = quiz_rec_guard.students.values().any(|s| s.rank.is_some());
    let result_cols = quiz_rec_guard.categories.len();

    rsx! {
        div {
            class: "flex flex-col w-full items-center gap-0.25 pt-5",
            div {
                class: "text-lg font-semibold",
                "{quiz_rec_guard.name}"
            }
            div { "{quiz_rec_guard.path}" }
            div { class: "flex w-full justify-end", { t!("date-stamp", date = quiz_rec_guard.metadata.updated_at()) } }
        }

        div {
            class: "flex w-full h-min-0 w-min-0",
            table {
                class: "quiz-report-table table-zebra",
                thead {
                    tr {
                        if has_ranks {
                            th { class: "w-min text-center", { t!("rank") } }
                        }
                        th { class: "max-w-none text-center", { t!("fullname") } }
                        if result_cols > 1 {
                            for category in quiz_rec_guard.categories.values() {
                                th { class: "rotated", "{category.name}" }
                            }
                        }
                        th { class: "rotated font-bold", { t!("total-grade") } }
                    }
                }
                tbody {
                    for (student_idx, student) in quiz_rec_guard.students.values().enumerate() {
                        tr {
                            class: if state.peek().is_supervisor && student.grade > 0 {
                                "cursor-pointer hover:bg-base-300"
                            } else { "" },
                            onclick: {
                                let student_id = student.id.clone();
                                let student_grade = student.grade;
                                move |_| {
                                    if state.peek().is_supervisor && student_grade > 0 {
                                        state.with_mut(|s| s.active_student = Some(student_id.clone()))
                                    }
                                }
                            },
                            if has_ranks && let Some(rank) = &student.rank {
                                td { class: "text-left", "{rank}" }
                            }
                            td { class: "text-left", "{student.name}" }
                            if student.grade == 0 {
                                for _ in 0..result_cols {
                                    td { { t!("uncertified-placeholder") } }
                                }
                                if result_cols > 1 {
                                    td { { t!("uncertified-placeholder") } }
                                }
                            } else if result_cols > 1 {
                                for i in 0..result_cols {
                                    td {
                                        {if state.read().as_percentage {
                                            quiz_rec_guard.results.get(student_idx, i).to_string()
                                        } else {
                                            quiz_rec_guard.grade.calc(*quiz_rec_guard.results.get(student_idx, i)).to_string()
                                        }}
                                    }
                                }
                                td {
                                    class: "font-semibold",
                                    {if state.read().as_percentage {
                                        format!("{:.0}", quiz_rec_guard.results.calc_row_average(student_idx))
                                    } else {
                                        student.grade.to_string()
                                    }}
                                }
                            } else {
                                td {
                                    class: "font-semibold",
                                    {if state.read().as_percentage{
                                        format!("{:.0}", quiz_rec_guard.results.calc_row_average(student_idx))
                                    } else {
                                        student.grade.to_string()
                                    }}
                                }
                            }
                        }
                    }
                }
            }
        }

        if state.read().students_details {
            StudentsReport {}
        }

        if state.read().stats {
            StatsReport {}
        }

        div {
            class: "flex flex-nowrap w-auto py-5",
            span { { t!("supervisor-sign") } }
        }

    }
}

#[component]
fn StatsReport() -> Element {
    #[derive(Default)]
    struct CategoryResult {
        pub name: String,
        pub grade_a: usize,
        pub grade_b: usize,
        pub grade_c: usize,
        pub grade_d: usize,
        pub average: f64,
    }
    let quiz_rec = use_context::<Signal<QuizRecord>>();
    let quiz_rec_guard = quiz_rec.read();
    let mut total = 0_usize;
    let mut res = quiz_rec_guard
        .categories
        .iter()
        .map(|(_, c)| CategoryResult {
            name: c.name.clone(),
            ..Default::default()
        })
        .collect::<Vec<_>>();

    for (student_idx, student) in quiz_rec_guard.students.values().enumerate() {
        if student.grade == 0 {
            continue;
        }
        for (i, &&score) in quiz_rec_guard
            .results
            .get_row(student_idx)
            .iter()
            .enumerate()
        {
            match quiz_rec_guard.grade.calc(score) {
                5 => res[i].grade_a += 1,
                4 => res[i].grade_b += 1,
                3 => res[i].grade_c += 1,
                2 => res[i].grade_d += 1,
                _ => (),
            }
        }
        total += 1;
    }
    if total == 0 {
        return rsx! {};
    }
    for cr in &mut res {
        cr.average = (cr.grade_a * 5 + cr.grade_b * 4 + cr.grade_c * 3 + cr.grade_d * 2) as f64
            / total as f64;
    }
    res.sort_unstable_by(|a, b| a.average.partial_cmp(&b.average).unwrap());
    let (total_grade_a, total_grade_b, total_grade_c, total_grade_d, total_average) =
        quiz_rec_guard.students.values().fold(
            (0, 0, 0, 0, 0),
            |(mut a, mut b, mut c, mut d, mut total), s| {
                match s.grade {
                    5 => a += 1,
                    4 => b += 1,
                    3 => c += 1,
                    2 => d += 1,
                    _ => (),
                }
                total += s.grade;

                (a, b, c, d, total)
            },
        );
    let is_single_cat = res.len() == 1;
    let total_students = quiz_rec_guard.students.len();

    rsx! {
        div {
            class: "flex flex-col w-full items-center gap-0.25 p-5",
            div { class: "text-lg font-semibold", { t!("stats") } }
        }
        div {
            class: "flex",

            table {
                class: "quiz-report-table table-zebra w-auto inline-table",
                thead {
                    tr {
                        th { class: "text-center", "" }
                        for cat in res.iter() {
                            th { class: "rotated", "{cat.name}" }
                        }
                        if !is_single_cat {
                            th { class: "rotated font-semibold", { t!("total-grade") } }
                        }
                    }
                }
                tbody {
                    tr {
                        td { class: "font-semibold text-left px-2", { t!("grade-d") } }
                        for cat in res.iter() {
                            td { "{cat.grade_d}" }
                        }
                        if !is_single_cat {
                            td { class: "font-semibold", "{total_grade_d}" }
                        }
                    }
                    tr {
                        td { class: "font-semibold text-left px-2", { t!("grade-c") } }
                        for cat in res.iter() {
                            td { "{cat.grade_c}" }
                        }
                        if !is_single_cat {
                            td { class: "font-semibold", "{total_grade_c}" }
                        }
                    }
                    tr {
                        td { class: "font-semibold text-left px-2", { t!("grade-b") } }
                        for cat in res.iter() {
                            td { "{cat.grade_b}" }
                        }
                        if !is_single_cat {
                            td { class: "font-semibold", "{total_grade_b}" }
                        }
                    }
                    tr {
                        td { class: "font-semibold text-left px-2", { t!("grade-a") } }
                        for cat in res.iter() {
                            td { "{cat.grade_a}" }
                        }
                        if !is_single_cat {
                            td { class: "font-semibold", "{total_grade_a}" }
                        }
                    }
                    tr {
                        td { class: "font-semibold text-left px-2", { t!("grade-average") } }
                        for cat in res.iter() {
                            td { class: "font-semibold", { format!("{:.1}", cat.average) } }
                        }
                        if !is_single_cat {
                            td { class: "font-bold", { format!("{:.1}", total_average as f64 / total as f64) } }
                        }
                    }
                }
            }
        }
        div {
            class: "flex pt-5",
            table {
                class: "table table-auto w-auto inline-table text-base",
                tr {
                    td { class: "p-1", { t!("stat-total") } }
                    td { class: "p-1 font-semibold border-1 px-2", "{total_students}" }
                    td { class: "p-1 pl-3", { t!("stat-in-fact") } }
                    td { class: "p-1 font-semibold border-1 px-2", "{total}" }
                    td { class: "p-1 pl-3", { t!("stat-certified") } }
                    td { class: "p-1 font-semibold border-1 px-2", "{total - total_grade_d}" }
                    td { class: "p-1 pl-3", { t!("stat-uncertified") } }
                    td { class: "p-1 font-semibold border-1 px-2", "{total_grade_d}" }
                }
            }
        }

        // div {
        //     class: "flex flex-nowrap w-auto my-5 gap-2",
        //     span { "За списком:" }
        //     span { class: "font-semibold mr-2 underline", "{total_students}" }
        //     span { "За фактом:" }
        //     span { class: "font-semibold mr-2 underline", "{total}" }
        //     span { "Атестовано:" }
        //     span { class: "font-semibold mr-2 underline", "{total - total_grade_d}" }
        //     span { "Не атестовано:" }
        //     span { class: "font-semibold mr-2 underline", "{total_grade_d}" }
        // }


        // div {
        //     class: "flex",
        //     table {
        //         class: "quiz-report-table table-zebra w-auto inline-table",
        //         thead {
        //             tr {
        //                 th { class: "text-center", { t!("categories") } }
        //                 th { class: "rotated", { t!("grade-a") } }
        //                 th { class: "rotated", { t!("grade-b") } }
        //                 th { class: "rotated", { t!("grade-c") } }
        //                 th { class: "rotated", { t!("grade-d") } }
        //                 th { class: "rotated", { t!("grade-average") } }
        //             }
        //         }
        //         tbody {
        //             for cat in res.iter().rev() {
        //                 tr {
        //                     td { class: "text-left", "{cat.name}" }
        //                     td { "{cat.grade_a}" }
        //                     td { "{cat.grade_b}" }
        //                     td { "{cat.grade_c}" }
        //                     td { "{cat.grade_d}" }
        //                     td { class: "font-semibold", { format!("{:.1}", cat.average) } }
        //                 }
        //             }
        //         }
        //     }
        // }
    }
}

#[component]
fn StudentsReport() -> Element {
    let quiz_rec = use_context::<Signal<QuizRecord>>();
    let absent_students = quiz_rec
        .read()
        .students
        .values()
        .filter(|s| s.grade == 0)
        .cloned()
        .collect::<Vec<_>>();
    let absent_total = absent_students.len();
    let uncertified_students = quiz_rec
        .read()
        .students
        .values()
        .filter(|s| s.grade == 2)
        .cloned()
        .collect::<Vec<_>>();
    let uncertified_total = uncertified_students.len();

    rsx! {
        if absent_total > 0 {
            div {
                class: "flex flex-col w-full items-center gap-0.25 p-5",
                div { class: "text-lg font-semibold", { t!("absent") } }
            }
            div {
                class: "flex flex-wrap gap-0.5",
                for (idx, student) in absent_students.iter().enumerate() {
                    span {
                        if let Some(rank) = &student.rank {
                            "{rank} {student.name}"
                        } else {
                            "{student.name}"
                        }
                        if idx < absent_total - 1 {
                            ", "
                        } else {
                            "."
                        }
                    }
                }
            }
        }
        if uncertified_total > 0 {
            div {
                class: "flex flex-col w-full items-center gap-0.25 p-5",
                div { class: "text-lg font-semibold", { t!("uncertified") } }
            }
            div {
                class: "flex flex-wrap gap-0.5",
                for (idx, student) in uncertified_students.iter().enumerate() {
                    span {
                        if let Some(rank) = &student.rank {
                            "{rank} {student.name}"
                        } else {
                            "{student.name}"
                        }
                        if idx < uncertified_total - 1 {
                            ", "
                        } else {
                            "."
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderStudentReport() -> Element {
    let mut state = use_context::<Signal<QuizReportState>>();
    // let quiz = use_context::<Signal<Quiz>>();
    // let quiz_guard = quiz.read();
    let quiz_rec = use_context::<Signal<QuizRecord>>();
    let quiz_rec_guard = quiz_rec.read();
    let mut student = use_signal(QuizRecordStudent::default);
    let mut answered = use_context_provider(|| Signal::new(Vec::new()));

    use_hook(move || {
        if let Some(student_id) = &state.read().active_student
            && let Some(student_rec) = quiz_rec.read().students.get(student_id)
        {
            let quiz_rec_guard = quiz_rec.read();
            student.set(student_rec.clone());
            let student_idx = quiz_rec_guard
                .students
                .iter()
                .position(|(id, _)| id == &student_rec.id)
                .unwrap_or_default();
            let answers = quiz_rec_guard
                .answers
                .get_row(student_idx)
                .iter()
                .map(|&a| a.clone())
                .collect::<Vec<_>>();
            answered.set(answers)
        } else {
            state.with_mut(|s| s.active_student = None);
        }
    });

    if student.read().id.is_empty() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "flex flex-col w-full items-center gap-0.25 pt-5",
            div {
                class: "text-lg font-semibold",
                "{quiz_rec_guard.name}"
            }
            div {
                if let Some(rank) = &student.read().rank {
                    "{rank} {student.read().name}"
                } else {
                    "{student.read().name}"
                }
            }
            div { class: "flex w-full justify-end", { t!("date-stamp", date = quiz_rec_guard.metadata.updated_at()) } }
        }

        for (idx, category) in quiz_rec_guard.categories.values().enumerate() {
            RenderStudentCategoryReport{ category_idx: idx, category_id: category.id.clone()}
        }
    }
}

#[component]
fn RenderStudentCategoryReport(category_idx: usize, category_id: String) -> Element {
    let quiz = use_context::<Signal<Quiz>>();
    let quiz_guard = quiz.read();
    let answered = use_context::<Signal<Vec<HashMap<String, HashSet<String>>>>>();

    let Some(category) = quiz_guard.categories.get(&category_id) else {
        return rsx! {};
    };
    let Some(answered_questions) = answered.read().get(category_idx).cloned() else {
        return rsx! {};
    };

    rsx! {
        div {
            class: "flex flex-col w-full items-center gap-0.25 pt-5 pb-1",
            div {
                class: "text-lg font-semibold",
                "{category.name}"
            }
        }
        div {
            class: "flex flex-col",
            for (question_id, answers_ids) in answered_questions.iter() {
                if let Some(question) = category.questions.get(question_id) {
                    RenderStudentQuestionReport { question: question.clone(), answers_ids: answers_ids.clone() }
                }
            }
        }
    }
}

#[component]
fn RenderStudentQuestionReport(question: QuizQuestion, answers_ids: HashSet<String>) -> Element {
    let quiz = use_context::<Signal<Quiz>>();
    let quiz_guard = quiz.read();
    let img_base_url = format!(
        "{}/images/{}/{}/",
        localhost(),
        quiz_guard.workspace,
        quiz_guard.id
    );

    let is_correct = question
        .answers
        .values()
        .filter(|a| a.correct)
        .map(|a| a.id.clone())
        .collect::<HashSet<String>>()
        == answers_ids;

    rsx! {
        div {
            class: "flex flex-col",
            div {
                class: "flex gap-2",
                div {
                    class: "flex items-center justify-center",
                    if question.answers.len() == 1 {
                        i { class: "bi bi-openai text-base-content/70" }
                    } else if is_correct {
                        i { class: "bi bi-check-square text-green-700" }
                    } else {
                        i { class: "bi bi-x-square text-red-700" }
                    }
                }
                div {
                    class: "flex flex-col",
                    if question.img {
                        div {
                            class: "max-w-30 w-full p-2",
                            img { class: "w-full h-auto object-contain", src: format!("{img_base_url}/{id}.webp", id = question.id) }
                        }
                    }
                    "{question.name}"
                }
            }
            if question.answers.len() == 1 {
                ol {
                    class: "list-inside space-y-0.5 pl-4 pt-1 pb-3",
                    li {
                        class: "flex gap-2",
                        div {
                            class: "flex items-center justify-center w-6",
                            i { class: "bi bi-chat-right-text text-base-content/70" }
                        }
                        div {
                            class: "flex flex-col",
                            if let Some(answer) = question.answers.values().next() {
                                if answer.img {
                                    div {
                                        class: "max-w-30 w-full p-2",
                                        img { class: "w-full h-auto object-contain", src: format!("{img_base_url}/{id}.webp", id = answer.id) }
                                    }
                                }
                                "[{quiz_guard.grade.similarity.to_string()}%] {answer.name}"
                            }
                        }
                    }
                    li {
                        class: "flex gap-2",
                        if let Some(answer) = question.answers.values().next() {{
                            let mut correct = false;
                            let mut similarity = 0;
                            let mut text = String::new();
                            for s in answers_ids.iter() {
                                if s.starts_with(answer.id.as_str()) {
                                    let mut parts = s.splitn(3, '|');
                                    let _a = parts.next().unwrap_or_default();
                                    let b = parts.next().unwrap_or_default();
                                    let c = parts.next().unwrap_or_default();

                                    correct = c == "1";
                                    similarity = b.parse::<usize>().unwrap_or_default();
                                } else {
                                    text.push_str(s);
                                }
                            }
                            rsx! {
                                div {
                                    class: "flex items-center justify-center w-6",
                                    if correct {
                                        i { class: "bi bi-check-square text-green-700" }
                                    } else {
                                        i { class: "bi bi-x-square text-red-700" }
                                    }
                                }
                                div { "[{similarity.to_string()}%] {text}" }
                            }
                        }}
                    }
                }
            } else {
                ol {
                    class: "list-inside space-y-0.5 pl-4 pt-1 pb-3",
                    for answer in question.answers.values() {
                        li {
                            class: "flex gap-2",
                            div {
                                class: "flex items-center justify-center w-6",
                                if answers_ids.contains(&answer.id) && answer.correct {
                                    i { class: "bi bi-check-circle text-green-700" }
                                } else if answers_ids.contains(&answer.id) && !answer.correct {
                                    i { class: "bi bi-check text-red-700" }
                                } else if answer.correct {
                                    i { class: "bi bi-circle text-red-700" }
                                }
                            }
                            div {
                                class: "flex flex-col",
                                if answer.img {
                                    div {
                                        class: "max-w-30 w-full p-2",
                                        img { class: "w-full h-auto object-contain", src: format!("{img_base_url}/{id}.webp", id = answer.id) }
                                    }
                                }
                                "{answer.name}"
                            }
                        }
                    }
                }
            }
        }
    }
}
