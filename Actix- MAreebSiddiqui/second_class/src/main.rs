//Social media

//user can sign in 
//user can see all videos
//user can see videos

use actix_web:: {HttpResponse, Responder,HttpServer,App,web,Result};
use actix_web:: {get,post};
use serde::Deserialize;

#[get("/video")]
async fn get_all_video() -> impl Responder{
    HttpResponse::Ok().body("ALL VIDEOS!")
}

#[get("/video/{id}")]
async fn get_video(v: web::Path<(u32,)>) -> impl Responder{
    HttpResponse::Ok().body(format!("Video ID: {}", v.0))
}

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(l: web::Json<Login>) -> Result<String> {
    if l.username == "Areeb" && l.password == "12345" {
    Ok(format!("User successfully signed in as {}", l.username))
    }
    else {
        Ok(format!("User {} is not Authorized",l.username))
    }

}



#[actix_rt::main]
async fn main () -> std::io::Result<()> {
   HttpServer::new(|| {
       App::new()
        .service(get_all_video)
        .service(get_video)
        .service(login)
   })
   .bind("127.0.0.1:5000")?
   .run()
   .await
 
}



