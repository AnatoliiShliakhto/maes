use crate::{components::dialogs::*, prelude::*, services::*};
use ::std::collections::HashSet;

#[component]
pub fn StudentsList() -> Element {
    let selected = use_context::<Signal<SelectedItem>>();
    let mut students = use_context_provider(|| Signal::new(Vec::<Student>::new()));
    let mut add_student_dialog = use_context::<AddStudentDialog>();

    use_effect(move || {
        api_fetch!(
            GET,
            format!("/api/v1/students/{node}", node = selected.read().id),
            on_success = move |body: Vec<Student>| students.set(body),
        );
    });

    let add_action = use_callback(move |body: Vec<Student>| {
        students.with_mut(|s| s.extend(body));
    });

    let paste_action = use_callback(move |_| {
        let students_guard = students.read();
        let Ok(mut payload) = Clipboard::paste_students() else {
            return;
        };
        let existing: HashSet<(Option<String>, String)> = students_guard
            .iter()
            .map(|u| (u.rank.clone(), u.name.clone()))
            .collect();
        payload.retain(|s| !existing.contains(&(s.rank.clone(), s.name.clone())));
        api_fetch!(
            POST,
            format!("/api/v1/students/{node_id}", node_id = selected.read().id),
            payload,
            on_success = move |body: Vec<Student>| students.with_mut(|s| s.extend(body)),
        );
    });

    let clear_action = use_callback(move |_| {
        let callback = use_callback(move |_| {
            api_call!(
                DELETE,
                format!("/api/v1/students/{node_id}", node_id = selected.read().id),
                Vec::<String>::new(),
                on_success = move || students.set(Vec::<Student>::new()),
            )
        });
        use_dialog().warning(t!("clear-users-message"), Some(callback))
    });

    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 items-center h-10 space-between",
            i { class: "bi bi-three-dots-vertical" }
            div {
                class: "w-full",
                { t!("students") }
                " [{students.read().len()}]"
            }
            ul {
                class: "menu menu-horizontal p-0 m-0 text-base-content flex-nowrap",
                li {
                    button {
                        class: "hover:text-success",
                        onclick: move |_| {
                            if selected.read().id.is_empty() {
                                ToastService::error(t!("select-node-first"));
                                return
                            }
                            add_student_dialog.open(selected.read().id.clone(), Some(add_action))
                        },
                        i { class: "bi bi-plus" }
                    }
                }
                li {
                    button {
                        class: "hover:text-accent",
                        onclick: paste_action,
                        i { class: "bi bi-clipboard" }
                    }
                }
                li {
                    button {
                        class: "hover:text-error",
                        onclick: clear_action,
                        i { class: "bi bi-eraser" }
                    }
                }
            }
        }
        div {
            class: "h-0.25 bg-base-300 mx-4 my-1",
        }

        ul {
            class: "list flex-scrollable",
            for student in students().into_iter() {
                RenderStudentRow { key: "{student.id}", student }
            }
        }
    }
}

#[component]
fn RenderStudentRow(student: ReadOnlySignal<Student>) -> Element {
    let selected = use_context::<Signal<SelectedItem>>();
    let mut students = use_context::<Signal<Vec<Student>>>();
    let student_guard = student.read();
    let first_chars = extract_first_chars(&student_guard.name);

    let delete_action = move |_| {
        api_call!(
            DELETE,
            format!("/api/v1/students/{node_id}", node_id = selected.read().id),
            vec![student.read().id.clone()],
            on_success = move || {
                let student_guard = student.read();
                students.with_mut(|u| u.retain(|u| u.id != student_guard.id));
            }
        )
    };

    rsx! {
        li {
            class: "list-row hover:bg-base-200 rounded-none p-0 group",
            div {
                class: "avatar avatar-placeholder justify-center size-10 my-3 ml-3",
                div {
                    class: "flex w-10 rounded-full items-center justify-center bg-neutral/60 text-neutral-content",
                    span {
                        class: "text",
                        "{first_chars}"
                    }
                }
            }
            div {
                class: "flex flex-col justify-center my-3 gap-1",
                div {
                    class: "semibold",
                    "{student_guard.name}"
                }
                div {
                    class: "text-xs text-base-content/60",
                    if let Some(rank) = student_guard.rank.clone() {
                        "{rank}"
                    }
                }
            }
            div {
                class: "hidden group-hover:flex h-full w-14 items-center justify-center",
                class: "bg-error/50 hover:bg-error cursor-pointer",
                onclick: delete_action,
                i { class: "bi bi-trash text-lg text-error-content" }
            }
        }
    }
}
