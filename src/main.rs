use actix_web::{body, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::path::{Path};
use std::fs;
use maud::{html, PreEscaped};

mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .route("/", web::get().to(get_gl))
        .route("/styles/default/{file}", web::get().to(get_css))
        .route("/styles/{file}", web::get().to(get_static))
        .route("/static/scripts.js", web::get().to(get_js))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

async fn get_css(file: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(fs::read(Path::new(&format!("./styles/default/{}", file))).unwrap())
}

async fn get_static(file: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(fs::read(Path::new(&format!("./styles/{}", file))).unwrap())
}

async fn get_js() -> impl Responder {
    HttpResponse::Ok().body(fs::read(Path::new("./static/scripts.js")).unwrap())
}

async fn get_gl() -> impl Responder {
    let head = "<!DOCTYPE html>
    <html>
    <head>
    <meta http-equiv=\"Content-Type\" content=\"text/html;charset=utf-8\">
    <title>EXACHAN</title>
    <link rel=\"stylesheet\" href=\"/styles/default/style.css\" type=\"text/css\">
    <link rel=\"shortcut icon\" href=\"http://'.$_SERVER['SERVER_NAME'].'/favicon.ico\" type=\"image/x-icon\">
    <script type=\"text/javascript\" src=\"/static/scripts.js\"></script>
    </head>
    <body>";
    let div = "<div class=\"post\">
	<img src=\"/styles/default/logo.png\" style=\"width: 200px; height: 200px;\" class=\"text_image\" alt=\"\">
	<h2>эксаба</h2>
	<p>
		это новое поколение анонимных досок,
		проект призванный потеснить убогие кусабы и вакабы на анонимных просторах интернетов.<br>
		Доска запиливалась специально для анонимного общения с множеством пользовательских функций,
		уникальная защита от вайпа, отсутсвие капчи, полная анонимность постинга, автодогрузка новых комментов в треде итд.<br>
		<a href=\"admin/\">админка</a>
	</p>
	<div class=\"info\">We are Anonymous. We are Legion. We do not forgive. We do not forget. Expect us.</div>
</div>";
    let html = html! {
        (PreEscaped(head))
        (PreEscaped(div))
        (PreEscaped(templates::footer::FOOTER))
    };
    HttpResponse::Ok().body(html.0)
} 