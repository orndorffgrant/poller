use std::any;
use std::cmp::max;
use std::collections::HashMap;
use std::convert::TryInto;
use std::task::Poll;

use anyhow::Result;
use hex;
use rand::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use rand::rngs::StdRng;
use serde::Deserialize;
use sha2::{Sha512, Digest};
use sqlx::prelude::*;
use sqlx::SqlitePool;
use tide::Middleware;
use tide::Redirect;

use crate::templates::users::*;

fn hash_pass(password: &str, salt: &str) -> String {
    let password_salt = format!("{}-{}", password, salt);
    let mut sha = Sha512::new();
    sha.update(password_salt);
    hex::encode(sha.finalize())
}

pub(crate) async fn create_user(db: &SqlitePool, username: &str, password: &str, role: &str) -> anyhow::Result<()> {
    let rng = StdRng::from_entropy();
    let salt: String = rng
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let password_hash = hash_pass(password, &salt);
    sqlx::query!(
        r#"
        INSERT INTO users (
            name,
            pass,
            salt,
            role
        )
        VALUES (?1, ?2, ?3, ?4)
        "#,
        username,
        password_hash,
        salt,
        role,
    ).execute(db).await?;
    Ok(())
}

pub async fn login_page(request: crate::Request) -> tide::Result {
    let session = request.session();
    let role: Option<String> = session.get("role");
    match role {
        None => {
            Ok(LoginPage {
                html_title: "Log in".to_string(),
                wrong: false,
            }
            .into())
        },
        Some(_) => {
            Ok(Redirect::temporary("/").into())
        },
    }
}

#[derive(FromRow)]
struct LoginUserQueryResult {
    id: i64,
    pass: String,
    salt: String,
    role: String,
}
#[derive(Deserialize)]
struct LoginBody {
    name: String,
    pass: String,
}
pub async fn login(mut request: crate::Request) -> tide::Result {
    let body: LoginBody = request.body_json().await?;
    let user_opt = sqlx::query_as!(
        LoginUserQueryResult,
        r#"
        SELECT
            id,
            pass,
            salt,
            role
        FROM users
        WHERE name = ?1
        "#,
        body.name,
    )
    .fetch_optional(&request.state().db)
    .await?;
    match user_opt {
        None => {
            Ok(LoginPageForm {
                wrong: true,
            }
            .into())
        },
        Some(user) => {
            if hash_pass(&body.pass, &user.salt) != user.pass {
                Ok(LoginPageForm {
                    wrong: true,
                }
                .into())
            } else {
                let session = request.session_mut();
                session.insert("user_id", user.id)?;
                session.insert("role", &user.role)?;
                if user.role == "admin" {
                    Ok(tide::Response::builder(200)
                        .header("HX-Redirect", "/admin")
                        .build())
                } else {
                    Ok(tide::Response::builder(200)
                        .header("HX-Redirect", "/polls")
                        .build())
                }
            }
        },
    }
}

pub async fn logout(mut request: crate::Request) -> tide::Result {
    let session = request.session_mut();
    session.remove("user_id");
    session.remove("role");
    Ok(Redirect::temporary("/login").into())
}

pub async fn user_list_page(request: crate::Request) -> tide::Result {
    let session = request.session();
    let role: Option<String> = session.get("role");
    if role != Some("admin".to_string()) {
        return Ok(Redirect::temporary("/").into())
    }
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            name
        FROM users
        WHERE name != "admin"
        "#,
    )
    .fetch_all(&request.state().db)
    .await?;
    Ok(UserListPage {
        html_title: "User List".to_string(),
        users: users,
    }
    .into())
}

#[derive(Deserialize)]
struct NewUserBody {
    name: String,
    pass: String,
}
pub async fn new_user(mut request: crate::Request) -> tide::Result {
    let body: NewUserBody = request.body_json().await?;
    let session = request.session();
    let role: Option<String> = session.get("role");
    if role != Some("admin".to_string()) {
        return Ok(Redirect::temporary("/").into())
    }
    create_user(&request.state().db, &body.name, &body.pass, "creator").await?;
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            name
        FROM users
        WHERE name != "admin"
        "#,
    )
    .fetch_all(&request.state().db)
    .await?;
    Ok(UserList {
        users: users,
    }
    .into())
}