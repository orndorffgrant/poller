use tide::Redirect;

use crate::templates::home::*;

pub async fn root(request: crate::Request) -> tide::Result {
    let session = request.session();
    let role: Option<String> = session.get("role");
    match role {
        None => {
            Ok(Redirect::temporary("/hello").into())
        },
        Some(role) => {
            if role == "admin" {
                Ok(Redirect::temporary("/admin").into())
            } else {
                Ok(Redirect::temporary("/polls").into())
            }
        },
    }
}

pub async fn hello(_request: crate::Request) -> tide::Result {
    Ok(HomeTemplate {
        html_title: "Home".to_string(),
    }
    .into())
}