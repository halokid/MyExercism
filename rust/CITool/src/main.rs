use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use std::{env, thread};
use std::path::Path;
use std::process::Command;
use std::time;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use env_logger::Env;

// #[derive(Debug)]
// struct SyncRepoParams {
//   repo: String,
//   token: String,
// }

fn get_tokens() -> Vec<String> {
  let mut tokens = Vec::new();
  tokens.push("xxx".to_string());
  tokens.push("yyy".to_string());
  tokens.push("zzz".to_string());
  tokens
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
  format!("Hello {name}!")
}

async fn not_found() -> impl Responder {
  format!("Error 404")
}

#[get("/syncrepo/{repo}/{token}")]
async fn syncrepo(path: web::Path<(String, String)>) -> impl Responder {
// async fn syncrepo(repo: web::Path<String>, token: web::Path<String>) -> impl Responder {
// async fn syncrepo(params: web::Path<SyncRepoParams>) -> impl Responder {
//   let repo = &params.repo;
//   let token = &params.token;
  // println!("Sync repo: {repo}, token: {token}");

  let (repo, token) = path.into_inner();
  println!("Sync repo: {}, token: {}", repo, token);

  // let tokens = get_tokens();
  // if !tokens.contains(&token) {
  //   return format!("Error Wrong token: {}", token);
  // }

  // let current_dir = env::current_dir().unwrap().display().to_string();
  // println!("current dir: {}", current_dir);

  let exec_path = env::current_exe().unwrap().parent().unwrap().display().to_string();
  println!("current dir: {}", exec_path);

  let jf = std::fs::read_to_string(exec_path + "/config.json").unwrap();
  let config: serde_json::Value = serde_json::from_str(&jf).unwrap();

  let tokens =  config.get("tokens").unwrap().as_array().unwrap();
  let mut tokens_str = Vec::new();
  for token in tokens {
    let tmp_token = token.as_str().unwrap().to_string();
    tokens_str.push(tmp_token);
  }

  if !tokens_str.contains(&token) {
    return format!("Error Wrong token: {}", token);
  }

  let repo_path = config.get("repo-base-path").unwrap().as_str().unwrap();
  println!("repo_path: {:?}", repo_path);

  // let repo = repo.into_inner();
  let repo_path_sync = repo_path.to_string() + repo.as_str();

  let root_path = Path::new(repo_path_sync.as_str());
  let set_path_ok = env::set_current_dir(&root_path);
  match set_path_ok {
    Ok(_) => {
      println!("Successfully changed repo folder {}", root_path.display());
      let output = Command::new("git").arg("pull").output().expect("[ERROR]: fail to run sync");

      if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Sync output:\n{}", stdout);
        // return format!("Sync output: {}", stdout);
      } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error Sync output:\n{}", stderr);
        return format!("Error Sync output:\n{}", stderr);
      }

      thread::sleep(time::Duration::from_millis(100));
      let commit_level = config.get("commit-log-level").unwrap().as_str().unwrap();
      let output = Command::new("git").arg("log").arg("-n").arg(commit_level).output().expect("[ERROR]: fail to run sync");
      if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Sync commit log output:\n{}", stdout);
        format!("Sync commit log output:\n{}", stdout)
      } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error Sync commit log output:\n{}", stderr);
        format!("Error Sync commit log output:\n{}", stderr)
      }

      // ()
    }
    Err(_) => {
      println!("[ERROR]: Can not find root path");
      format!("[ERROR]: Can not find root path {:?}", root_path)
    }
  }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
  env_logger::Builder::from_env(Env::default()
    .default_filter_or("debug"))
    .init();

  HttpServer::new(|| {
    App::new()
      .wrap(Logger::default())
      .service(greet)
      .service(syncrepo)
      .default_service(web::route().to(not_found))
  })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

