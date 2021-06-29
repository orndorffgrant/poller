use std::cmp::max;
use std::collections::HashMap;
use std::convert::TryInto;

use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use sqlx::prelude::*;
use sqlx::SqlitePool;
use tide::Redirect;

use crate::templates::users::*;

pub async fn login_page(request: crate::Request) -> tide::Result {
    Ok(LoginPage {
        html_title: "Log in".to_string(),
    }
    .into())
}

pub async fn login(request: crate::Request) -> tide::Result {
    Ok(LoginPage {
        html_title: "Log in".to_string(),
    }
    .into())
}