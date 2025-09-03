use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    user_name: String,
    user_email: String,
}



pub async fn index() -> Html<String> {
    let template = IndexTemplate {
        title: "爱情记录".to_string(),
        user_name: "云深".to_string(),
        user_email: "yunshen47672@gmail.com".to_string(),
    };
    match template.render() {
        Ok(rendered) => Html(rendered),
        Err(e) => Html(format!("Error rendering template: {}", e)),
    }
}