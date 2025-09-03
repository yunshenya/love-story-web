use askama::Template;
use axum::debug_handler;
use axum::response::Html;


#[derive(Template)]
#[template(path = "auth.html")]
struct AuthTemplate {
    title: String,
}

#[debug_handler]
pub async fn auth() -> Html<String> {
    let template = AuthTemplate {
        title: "love".to_string(),
    };
    match template.render() {
        Ok(rendered) => Html(rendered),
        Err(e) => Html(format!("Error rendering template: {}", e)),
    }
}