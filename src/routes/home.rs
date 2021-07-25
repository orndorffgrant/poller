use tide::Redirect;

pub async fn root(request: crate::Request) -> tide::Result {
    let session = request.session();
    let role: Option<String> = session.get("role");
    match role {
        None => Ok(Redirect::temporary("/login").into()),
        Some(role) => {
            if role == "admin" {
                Ok(Redirect::temporary("/admin").into())
            } else {
                Ok(Redirect::temporary("/polls").into())
            }
        }
    }
}
