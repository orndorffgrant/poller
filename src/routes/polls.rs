use crate::records::{Poll, PartialPoll};
use crate::templates::polls::*;
use crate::utils;
use sqlx::prelude::*;
use rand::{Rng, distributions::Alphanumeric};

pub async fn index(request: crate::Request) -> tide::Result {
    let polls = Poll::all().fetch_all(&request.state().db).await?;
    Ok(IndexTemplate::for_polls(polls.as_slice()).into())
}

pub async fn show(request: crate::Request) -> tide::Result {
    let poll = Poll::find_by_id(request.param("poll_id")?.parse()?)
        .fetch_one(&request.state().db)
        .await?;

    Ok(ShowTemplate::for_poll(&poll).into())
}

pub async fn delete(request: crate::Request) -> tide::Result {
    Poll::delete_by_id(request.param("poll_id")?.parse()?)
        .execute(&request.state().db)
        .await?;

    // if we had sessions, we'd set a flash message with whether this was successful
    Ok(tide::Redirect::new("/").into())
}

pub async fn update(mut request: crate::Request) -> tide::Result {
    let poll: PartialPoll = utils::deserialize_body(&mut request).await?;
    let poll_id = request.param("poll_id")?.parse()?;
    let rows_updated = poll
        .update_by_id(poll_id)
        .execute(&request.state().db)
        .await?;

    if rows_updated.rows_affected() == 1 {
        Ok(tide::Redirect::new(format!("/polls/{}", poll_id)).into())
    } else {
        Ok(PollForm::for_partial_poll(&poll).into())
    }
}

pub async fn create(mut request: crate::Request) -> tide::Result {
    let db = &request.state().db;
    let mut tx = db.begin().await?;
    let poll: PartialPoll = utils::deserialize_body(&mut request).await?;
    let created = poll.create().execute(&mut tx).await?;

    if created.rows_affected() == 1 {
        let (last_id,) = Poll::last_id().fetch_one(&mut tx).await?;
        tx.commit().await?;

        Ok(tide::Redirect::new(format!("/poll/{}", last_id)).into())
    } else {
        Ok(PollForm::for_partial_poll(&poll).into())
    }
}

pub async fn new(_request: crate::Request) -> tide::Result {
    let poll = PartialPoll::default();
    let new_id: String = rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
    Ok(PollForm::for_partial_poll(&poll).into())
}
