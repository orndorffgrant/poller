use crate::records::{Poll, PartialPoll};
use askama::Template;

#[derive(Template)]
#[template(path = "polls/form.html")]
pub struct PollForm<'a> {
    title: &'a str,
    text: &'a str,
    action: String,
}

impl<'a> PollForm<'a> {
    pub fn for_partial_poll(poll: &'a PartialPoll) -> Self {
        Self {
            title: poll.title.as_deref().unwrap_or_default(),
            text: poll.text.as_deref().unwrap_or_default(),
            action: "/polls".into(),
        }
    }

    // pub fn for_poll(poll: &'a Poll) -> Self {
    //     Self {
    //         title: &poll.title,
    //         text: &poll.text,
    //         action: format!("/poll/{}", poll.id),
    //     }
    // }
}

#[derive(Template)]
#[template(path = "polls/index.html")]
pub struct IndexTemplate<'a> {
    polls: &'a [Poll],
}

impl<'a> IndexTemplate<'a> {
    pub fn for_polls(polls: &'a [Poll]) -> Self {
        Self { polls }
    }
}

#[derive(Template)]
#[template(path = "polls/show.html")]
pub struct ShowTemplate<'a> {
    title: &'a str,
    text: &'a str,
}

impl<'a> ShowTemplate<'a> {
    pub fn for_poll(poll: &'a Poll) -> Self {
        Self {
            title: &poll.title,
            text: &poll.text,
        }
    }
}
