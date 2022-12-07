use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

// GET ********************************************************
#[get("/retorna_format")]
async fn retorna_format() -> impl Responder {
    format!("Retorna: format!")
}

#[get("/retorna_ok_1")]
async fn retorna_ok_1() -> impl Responder {
    HttpResponse::Ok().body("okkkk")
}
#[get("/retorna_ok_2")]
async fn retorna_ok_2() -> actix_web::Result<impl Responder> {
    Ok("Retorna: Ok")
}

#[get("/recibe_parametros_simple/{name}")]
async fn recibe_parametros_simple(name: web::Path<String>) -> impl Responder {
    format!("Hola {name}")
}
#[get("/{id}/{name}/recibe_parametros.html")]
async fn recibe_parametros(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    format!("Parametros enviados: {}! id:{}", name, id)
}

// POST ********************************************************
// Json Body: cogue todo el contenido dentro de req_body
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// FORM DATA: prueba para envio de datos de los formularios
#[derive(Deserialize)]
struct Info {
    username: String,
}
#[post("/data_form")]
async fn data_form(form: web::Form<Info>) -> Result<String> {
    // La data se debe enviar como Form Data, no como Json en el Body
    Ok(format!("Welcome {}!", form.username))
}

 

// MAIN configuration ********************************************************
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::FormConfig::default().limit(4096))
            // GET
            // ruta en route
            .route("/inicio", web::get().to(|| async { "Iniciooo |" }))
            //ruta como atributo
            .service(recibe_parametros)
            .service(retorna_format)
            .service(retorna_ok_1)
            .service(retorna_ok_2)
            .service(recibe_parametros_simple)
            // POST
            .service(echo)
            .service(data_form)
    })
    .bind("127.0.0.1:5050")?
    .run()
    .await
}
