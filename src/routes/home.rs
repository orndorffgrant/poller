use crate::templates::home::*;

pub async fn home(_request: crate::Request) -> tide::Result {
    Ok(HomeTemplate::new("Poller").into())
}