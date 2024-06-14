use axum::http::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::{self, header, Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
};

static URL: &str = "http://127.0.0.1:3000";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Serialize, Deserialize)]
pub struct UserIn {
    pub username: String,
    pub password: String,
}

async fn log_result(req: reqwest::RequestBuilder, res: Response) {
    let req = req.build().unwrap();
    let req_headers = req.headers().clone();
    let res_headers = res.headers();
    //let res_keys: Vec<String> = res_headers.keys().map(|key| key.to_string()).collect();
    //let res_values: Vec<String> = res_headers
    //   .values()
    //    .map(|value| value.to_str().unwrap().to_string())
    //     .collect();
    //let res_headers:Vec<(String,String)> = res_keys.iter().map(|x|*x).zip(res_values).collect();
    //let test = res_headers.iter().map(
    //   |f| format!("\t{}\t{}\n", f.0.as_str(), f.1.to_str().unwrap()), //        f.0.to_string()+"\t"+f.1.to_str().unwrap()
    //);
    //let test1: Vec<String> = test.collect();
    //let test:Vec<String,String>=res_headers.collect();
    let mut req_headers_str = String::new();
    let mut res_headers_str = String::new();

    for i in req_headers {
        req_headers_str = format!(
            "\t➤  {}: {}\n{}",
            i.0.unwrap(),
            i.1.to_str().unwrap(),
            req_headers_str
        );
    }
    for i in res_headers {
        res_headers_str = format!(
            "\t➤  {}: {}\n{}",
            i.0.to_string(),
            i.1.to_str().unwrap(),
            res_headers_str
        );
    }
    let body = reqwest::Body::from("None".to_string());
    println!(
        "══════════════───────────────────────────────────────
Request  :
 🡆  Url    \t: {}
 🡆  version\t: {:#?}
 🡆  Method \t: {}
 🡆  Headers \n{}
 🡆  Body   \t: {:?}

 Response :
 🡆  status \t: {}
 🡆  Headers \n{}
 🡆  body   \t: {}",
        req.url().to_string(),
        req.version(),
        req.method(),
        req_headers_str,
        req.body().unwrap_or_else(|| &body),
        res.status(),
        res_headers_str,
        res.text().await.unwrap_or("None".to_string()) // res.text().await.unwrap_or("None".to_string())
    )
}
fn value() -> serde_json::Value {
    json!({
        "username":"defalt",
        "password":"6367411"
    })
}

async fn create_client() -> Result<Client, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.append(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.append(header::USER_AGENT, HeaderValue::from_static("mozilla"));
    headers.append(
        header::AUTHORIZATION,
        HeaderValue::from_static("Bearer jibrish"),
    );

    reqwest::Client::builder()
        .default_headers(headers.clone())
        .connection_verbose(true)
        .referer(true)
        .local_address(IpAddr::V4(Ipv4Addr::LOCALHOST))
        .user_agent(APP_USER_AGENT)
        .build()
}
#[tokio::test]
async fn test_user_details_get() -> Result<(), reqwest::Error> {
    let req = create_client().await?.get(URL.to_owned() + ":3000/api/v1");
    let clone = req.try_clone().unwrap();
    let res = req.bearer_auth("admin").send().await.unwrap();
    log_result(clone, res).await;
    Ok(())
}

#[tokio::test]
async fn test_login_post_json_admin() -> Result<(), reqwest::Error> {
    let req = create_client()
        .await?
        .post(URL.to_owned() + "/api/login")
        .json(&value());

    //        .bearer_auth("new_user.username".to_string())
    let t = req.try_clone().unwrap();
    let res = req.send().await.unwrap();
    log_result(t, res).await;
    Ok(())
}
#[tokio::test]
async fn test_register_post_json() -> Result<(), reqwest::Error> {
    let req = create_client()
        .await?
        .post(URL.to_owned() + "/api/register")
        .json(&value());
    //       .bearer_auth("new_user.username".to_string())
    let clone = req.try_clone().unwrap();
    let res = req.send().await.unwrap();
    log_result(clone, res).await;
    Ok(())
}
#[tokio::test]
async fn test_register_post_form_hash() -> Result<(), reqwest::Error> {
    let mut form_hash = HashMap::new();
    form_hash.insert("username", "test");
    form_hash.insert("password", "test");
    let req = create_client()
        .await?
        .post(URL.to_owned() + "/api/v1/auth/register")
        .json(&form_hash);
    let clone = req.try_clone().unwrap();
    let res = req.send().await.unwrap();
    log_result(clone, res).await;
    Ok(())
}
#[tokio::test]
async fn new_user() -> Result<(), reqwest::Error> {
    let mut form_hash = HashMap::new();
    form_hash.insert("username", "defalt");
    form_hash.insert("password", "6367411");

    let new_user_req = create_client()
        .await?
        .post(URL.to_owned() + "/api/v1/auth/register")
        .form(&form_hash);
    let new_user_req_clone = new_user_req.try_clone().unwrap();
    let new_user_res = new_user_req.send().await.unwrap();

    let new_user: UserIn = new_user_res.json::<UserIn>().await.unwrap();

    //    log_result(new_user_req_clone, new_user_res).await;

    let login_post_json_second_req = create_client()
        .await?
        .post(URL.to_owned() + "/api/login")
        .json(&json!({"password":new_user.password,"username":new_user.username}));
    let login_with_credentials_res = new_user_req_clone.send().await.unwrap();

    log_result(login_post_json_second_req, login_with_credentials_res).await;

    let password = new_user.password;

    let user_details_get_second_req = create_client()
        .await?
        .get(URL.to_owned() + format!("/api/{password}").as_str())
        .bearer_auth(new_user.username);
    let clone_user_details_get_second_req = user_details_get_second_req.try_clone().unwrap();
    let user_details_get_second_res = clone_user_details_get_second_req.send().await.unwrap();
    log_result(user_details_get_second_req, user_details_get_second_res).await;
    Ok(())
}
//#[tokio::test]
//async fn quick_dev() -> Result<(), Error> {
// let reqwest_form = Form::new()
//         .text("username", "defalt hash")
//         .text("id", "1");
//     test_register_post_reqwest_form
// let test_register_post_reqwest_form = create_client()
//     .post(URL.to_owned() + "/api/register")
//     .multipart(reqwest_form);

// log_result(_test_register_post_json).await;
// log_result(test_register_post_reqwest_form).await;

// println!("form: {:?}",form);
//   Ok(())
//}
