use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use serde_json::json;
use tera::Tera;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct MetaTag {
    name: String,
    content: String
}

#[derive(Serialize, Deserialize)]
struct TagLevel {
    title: String,
    level: i64
}

#[derive(Serialize, Deserialize)]
struct Skills {
    name: String,
    title: String,
    description: Option<String>,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Experience {
    name: String,
    title: String,
    company: String,
    location: String,
    start_date: String,
    end_date: Option<String>,
    is_current: bool,
    responsibilities: Vec<String>,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct Degree {
    name: String,
    degree: String,
    institution: String,
    location: String,
    year: i64,
    achievements: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Certification {
    name: String,
    issuer: String,
    year: i64,
}

#[derive(Serialize, Deserialize)]
struct Project {
    title: String,
    role: String,
    year: i64,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Link {
    name: String,
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct Personal {
    name: String,
    title: String,
    subtitle: String,
    location: String,
    links: Vec<Link>,
}

#[derive(Serialize, Deserialize)]
struct ParsedResume {
    personal: Personal,
    summary: String,
    skills: Vec<Skills>,
    experiences: Vec<Experience>,
    degrees: Vec<Degree>,
    certifications: Vec<Certification>,
}

// Load json from ./data/resume.json and return it at /resume
#[get("/resume")]
async fn resume() -> impl Responder {
    let resume = fs::read_to_string("../data/resume.json").unwrap();

    let parsed: ParsedResume = serde_json::from_str(&resume).unwrap();

    let json = json!({
        "status": "ok",
        "message": {
            "resume": parsed
        }
    });
    
    HttpResponse::Ok().json(json)
}

#[get("/")]
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("title", "Verron Knowles - Resume");

    let meta_tags = vec![
        MetaTag {
            name: "description".to_string(),
            content: "A simple resume API".to_string()
        },
        MetaTag {
            name: "keywords".to_string(),
            content: "resume, api, actix, rust".to_string()
        }
    ];

    context.insert("meta_tags", &meta_tags);

    let loaded = include_str!("../data/resume.json");

    let parsed: ParsedResume = serde_json::from_str(loaded).unwrap();

    context.insert("personal", &parsed.personal);
    context.insert("summary", &parsed.summary);
    context.insert("skills", &parsed.skills);
    context.insert("experiences", &parsed.experiences);
    context.insert("degrees", &parsed.degrees);
    context.insert("certifications", &parsed.certifications);

    let rendered = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(index)
            .service(resume)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
