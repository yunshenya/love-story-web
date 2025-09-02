use askama::Template;
use axum::debug_handler;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    name: String,
}

#[debug_handler]
pub async fn index() -> Html<String> {
    let template = Index {
        name: "云深".to_string(),
    };

    match template.render() {
        Ok(rendered) => Html(rendered),
        Err(e) => Html(format!("Error rendering template: {}", e)),
    }
}
