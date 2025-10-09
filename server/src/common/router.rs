use crate::{middleware::*, handlers::*};
use ::axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::{delete, get, patch, post},
};
use ::shared::{models::ServerConfig, utils::parse_scheme_host_port};
use ::std::path::PathBuf;
use ::tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};
use ::tower::ServiceBuilder;

pub fn init_router(path: PathBuf, config: &ServerConfig) -> Router {
    let (scheme, host, port) = parse_scheme_host_port(config.host.as_str()).unwrap();
    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_str(&format!("http://127.0.0.1:{port}"))
                .unwrap_or_else(|_| HeaderValue::from_static("http://127.0.0.1:4583")),
            HeaderValue::from_str(&format!("{scheme}://{host}:{port}"))
                .unwrap_or_else(|_| HeaderValue::from_static("http://192.168.137.1:4583")),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .fallback_service(
            ServeDir::new(path.join("client")).fallback(ServeFile::new(path.join("client").join("index.html"))),
        )
        .route("/health", get(liveness))
        .nest("/api/v1", api_v1_router())
        .layer(
            ServiceBuilder::new().layer(StaticHeadersLayer::new(
                "no-store, no-cache, must-revalidate",
                "timeout=60, max=1000",
            )),
        )
        .layer(cors)
}

fn auth_router() -> Router {
    Router::new().route("/", post(authorize).delete(logout))
}

fn api_v1_router() -> Router {
    Router::new()
        .nest("/auth", auth_router())
        .nest("/workspaces", workspace_manager_router())
        .nest("/entities", entity_router())
        .nest("/tasks", task_manager_router())
        .nest("/students", students_manager_router())
        .nest("/activities", activities_router())
        .nest("/manager/quizzes", quiz_manager_router())
        .nest("/manager/surveys", survey_manager_router())
        .nest("/manager/images", image_manager_router())
}

fn entity_router() -> Router {
    Router::new()
        .route(
            "/{kind}/{id}",
            get(list_entities_by_node).delete(delete_entity),
        )
        .route("/{kind}", get(list_entities))
}

fn workspace_manager_router() -> Router {
    Router::new()
        .route(
            "/tree/{kind}/{node_id}",
            patch(update_workspace_treenode).delete(delete_workspace_treenode),
        )
        .route(
            "/users/{user_id}",
            get(list_workspace_users_by_node).delete(delete_workspace_user),
        )
        .route(
            "/tree/{kind}",
            get(get_workspace_tree).post(create_workspace_treenode),
        )
        .route("/users", get(list_workspace_users).post(add_workspace_user))
        .route("/{ws_id}", delete(delete_workspace))
        .route("/", get(list_workspaces).post(create_workspace))
}

fn quiz_manager_router() -> Router {
    Router::new()
        .route(
            "/{quiz_id}/{category_id}/{question_id}",
            patch(update_quiz_question).delete(delete_quiz_question),
        )
        .route(
            "/{quiz_id}/{category_id}",
            patch(update_quiz_category)
                .delete(delete_quiz_category)
                .post(create_quiz_question),
        )
        .route(
            "/{quiz_id}",
            get(get_quiz)
                .patch(update_quiz)
                .delete(delete_quiz)
                .post(create_quiz_category),
        )
        .route("/", post(create_quiz))
}

fn survey_manager_router() -> Router {
    Router::new()
        .route(
            "/{survey_id}/{category_id}",
            patch(update_survey_category).delete(delete_survey_category),
        )
        .route(
            "/{survey_id}",
            get(get_survey)
                .patch(update_survey)
                .delete(delete_survey)
                .post(create_survey_category),
        )
        .route("/", post(create_survey))
}

fn students_manager_router() -> Router {
    Router::new()
        .route(
            "/{node_id}",
            get(list_students_by_node)
                .post(add_students)
                .delete(remove_students_by_node),
        )
        .route("/", get(list_students).delete(remove_students))
}

fn task_manager_router() -> Router {
    Router::new()
        .route("/categories/{kind}/{id}", get(get_task_categories))
        .route("/{kind}/{task_id}", get(get_task).delete(delete_task))
        .route("/{kind}", post(create_task))
        .route("/", get(list_tasks))
}

fn image_manager_router() -> Router {
    Router::new()
        .route("/validate/{kind}/{entity_id}", get(validate_images))
        .route("/{entity_id}/{item_id}", post(add_image).delete(remove_image))
}

fn activities_router() -> Router {
    Router::new()
        .route("/details/{workspace_id}/{task_id}", get(get_activity_details))
        .route("/details/{workspace_id}/{task_id}/{student_id}", get(get_activity_details_with_student))
        .route("/{workspace_id}/{task_id}", get(get_activity_details))
        .route("/{workspace_id}/{task_id}/{student_id}", get(get_activity_with_student))
        .route("/", post(update_activity))
}