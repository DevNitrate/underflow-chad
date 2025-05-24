use actix_files::Files;
use actix_web::{web::{self, Redirect}, App, Either, HttpResponse, HttpServer, Responder};
use dotenv::{dotenv, var};
use lazy_static::lazy_static;
use serde_json::json;
use supabase_rs::SupabaseClient;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let src: &str = "templates/**/*";
        let tera: Tera = Tera::new(src).unwrap();
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
    render_page("inspect.html")
}

#[actix_web::get("/crypt")]
async fn crypt() -> impl Responder {
    render_page("crypt.html")
}

#[actix_web::get("/source")]
async fn source() -> impl Responder {
    render_page("source.html")
}

#[actix_web::post("/source_next")]
async fn source_next() -> impl Responder {
    HttpResponse::Ok().body("/password")
}

#[actix_web::get("/password")]
async fn password() -> impl Responder {
    render_page("password.html")
}

#[actix_web::post("/password_check/{password_in}")]
async fn password_check(path: web::Path<String>) -> impl Responder {
    let password_in: String = path.into_inner();

    if password_in == "corner" {
        Either::Left(Redirect::to("/end").see_other())
    } else {
        Either::Right(HttpResponse::Ok().body("incorrect password"))
    }
}

#[actix_web::get("/1plus3times4")]
async fn equality() -> impl Responder {
    HttpResponse::Ok().body("a little bit of math...")
}

#[actix_web::get("/13")]
async fn thirteen() -> impl Responder {
    render_page("13.html")
}

#[actix_web::get("/end")]
async fn end() -> impl Responder {
    render_page("end.html")
}

#[actix_web::post("/key/{discord_in}/{key_in}")]
async fn key(path: web::Path<(String, String)>) -> impl Responder {
    let (discord_in, key_in): (String, String) = path.into_inner();

    dotenv().ok();

    
    if key_in == "oxide" {
        let supabase: SupabaseClient = SupabaseClient::new(var("SUPABASE_URL").unwrap(), var("SUPABASE_KEY").unwrap()).unwrap();

        let insert_discord: Result<(), String> = supabase.insert_without_defined_key("solvers", json!({
            "discord": discord_in
        })).await;

        match insert_discord {
            Ok(_) => {
                HttpResponse::Ok().json(json!({
                    "answer": "correct key ! let us know you finished on the discord server",
                    "color": "18ff65"
                }))
            },
            Err(e) => {
                println!("err: {}\n", e);
                HttpResponse::Ok().json(json!({
                    "answer": "error adding your discord to the database, please try again later and if the issue persists report it to @devnitrate on discord",
                    "color": "c92e2e"
                }))
            }
        }
    } else {
        HttpResponse::Ok().json(json!({
            "answer": "incorrect key",
            "color": "c92e2e"
        }))
    }
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;

    println!("server running");

    HttpServer::new(|| {
        App::new().default_service(web::to(not_found)).service(Files::new("/static", "./static")).service(index).service(binary).service(inspect).service(crypt).service(source).service(source_next).service(password).service(password_check).service(equality).service(thirteen).service(end).service(key)
    })
    .bind(("0.0.0.0", port)).unwrap()
    .run()
    .await
}
