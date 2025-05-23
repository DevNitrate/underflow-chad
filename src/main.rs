use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source: &str = "templates/**/*";
        let tera: Tera = Tera::new(source).unwrap();
        tera
    };
}

fn render_page(name: &str) -> HttpResponse {
    let context: tera::Context = tera::Context::new();

    match TEMPLATES.render(name, &context) {
        Ok(page) => {
            HttpResponse::Ok().body(page)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error loading page:\n\n{}", e))
        }
    }
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Requested page not found")
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    render_page("index.html")
}

#[actix_web::get("/bin")]
async fn binary() -> impl Responder {
    render_page("binary.html")
}

#[actix_web::get("/ins")]
async fn inspect() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;

    println!("server running");

    HttpServer::new(|| {
        App::new().default_service(web::to(not_found)).service(Files::new("/static", "./static")).service(index).service(binary)
    })
    .bind(("0.0.0.0", port)).unwrap()
    .run()
    .await
}
