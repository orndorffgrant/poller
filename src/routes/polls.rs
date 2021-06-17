use std::cmp::max;
use std::collections::HashMap;
use std::convert::TryInto;
use std::hash::Hash;

use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use sqlx::prelude::*;
use sqlx::SqlitePool;
use tide::Redirect;

use crate::templates::home::NotFoundTemplate;
use crate::templates::polls::*;

const POLL_TYPE_SINGLE: &str = "single";
const POLL_TYPE_MULTI: &str = "multi";
const POLL_TYPE_SCORE: &str = "score";

enum PollType {
    Single,
    Multi,
    Score,
}

impl PollType {
    fn from_string(str: &str) -> Result<PollType> {
        match str {
            POLL_TYPE_SINGLE => Ok(PollType::Single),
            POLL_TYPE_MULTI => Ok(PollType::Multi),
            POLL_TYPE_SCORE => Ok(PollType::Score),
            _ => Err(anyhow::Error::msg("invalid poll type")),
        }
    }
}

pub async fn new(request: crate::Request) -> tide::Result {
    let new_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    sqlx::query!(
        r#"
        INSERT INTO polls (
            id,
            title,
            description,
            poll_type
        )
        VALUES (?1, ?2, ?3, ?4)
        "#,
        new_id,
        "",
        "",
        POLL_TYPE_SINGLE
    )
    .execute(&request.state().db)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO options (name, order_index, poll_id)
        VALUES
            (?1, ?2, ?7),
            (?3, ?4, ?7),
            (?5, ?6, ?7)
        "#,
        "Option 1",
        1024,
        "Option 2",
        2048,
        "Option 3",
        3072,
        new_id,
    )
    .execute(&request.state().db)
    .await?;

    let uri: String = "/poll/".to_string() + &new_id + "/edit";
    Ok(Redirect::temporary(uri).into())
}

#[derive(FromRow)]
struct TakePagePollQueryResult {
    id: String,
    title: String,
    description: String,
    require_name: bool,
    allow_participant_options: bool,
    poll_type: String,
    published: bool,
}

pub async fn take_page(request: crate::Request) -> tide::Result {
    let id = request.param("poll_id")?;
    if request.cookie(id).is_some() {
        Ok(tide::Response::builder(400)
            .body("already submitted")
            .build())
    } else {
        let p_opt = sqlx::query_as!(
            TakePagePollQueryResult,
            r#"
        SELECT
            id,
            title,
            description,
            require_name,
            allow_participant_options,
            poll_type,
            published
        FROM polls
        WHERE id = ?1
        "#,
            id
        )
        .fetch_optional(&request.state().db)
        .await?;

        if let Some(p) = p_opt {
            if !p.published {
                Ok(tide::Response::builder(404).build())
            } else {
                let options = sqlx::query_as!(
                    EditPageOptionQueryResult,
                    r#"
                SELECT
                    id,
                    name
                FROM options
                WHERE poll_id = ?1
                ORDER BY order_index
                "#,
                    id
                )
                .fetch_all(&request.state().db)
                .await?;
                let html_title = if p.title.is_empty() {
                    "New Poll"
                } else {
                    &p.title
                };
                Ok(TakePage {
                    html_title: html_title.to_string(),
                    id: p.id,
                    title: p.title,
                    description: p.description,
                    require_name: p.require_name,
                    allow_participant_options: p.allow_participant_options,
                    poll_type: p.poll_type,
                    options: options
                        .iter()
                        .map(|o| EditPageOption {
                            id: o.id,
                            name: o.name.to_owned(),
                        })
                        .collect(),
                }
                .into())
            }
        } else {
            Ok(tide::Response::builder(404)
                .body(
                    NotFoundTemplate {
                        html_title: "Not Found".to_string(),
                    }
                    .to_string(),
                )
                .content_type(tide::http::mime::HTML)
                .build())
        }
    }
}

#[derive(FromRow)]
struct EditPagePollQueryResult {
    id: String,
    title: String,
    description: String,
    require_name: bool,
    allow_participant_options: bool,
    poll_type: String,
    published: bool,
}
#[derive(FromRow)]
struct EditPageOptionQueryResult {
    id: i64,
    name: String,
}
pub async fn edit_page(request: crate::Request) -> tide::Result {
    let id = request.param("poll_id")?;
    let p_opt = sqlx::query_as!(
        EditPagePollQueryResult,
        r#"
        SELECT
            id,
            title,
            description,
            require_name,
            allow_participant_options,
            poll_type,
            published
        FROM polls
        WHERE id = ?1
        "#,
        id
    )
    .fetch_optional(&request.state().db)
    .await?;

    if let Some(p) = p_opt {
        let options = sqlx::query_as!(
            EditPageOptionQueryResult,
            r#"
            SELECT
                id,
                name
            FROM options
            WHERE poll_id = ?1
            ORDER BY order_index
            "#,
            id
        )
        .fetch_all(&request.state().db)
        .await?;
        let html_title = if p.title.is_empty() {
            "New Poll"
        } else {
            &p.title
        };
        Ok(EditPage {
            html_title: html_title.to_string(),
            id: p.id,
            title: p.title,
            description: p.description,
            require_name: p.require_name,
            allow_participant_options: p.allow_participant_options,
            poll_type: p.poll_type,
            published: p.published,
            options: options
                .iter()
                .map(|o| EditPageOption {
                    id: o.id,
                    name: o.name.to_owned(),
                })
                .collect(),
        }
        .into())
    } else {
        Ok(tide::Response::builder(404)
            .body(
                NotFoundTemplate {
                    html_title: "Not Found".to_string(),
                }
                .to_string(),
            )
            .content_type(tide::http::mime::HTML)
            .build())
    }
}

#[derive(FromRow)]
struct IdRow {
    id: i64,
}
#[derive(Deserialize)]
struct SavePollBodyOption {
    id: i64,
    name: String,
}
#[derive(Deserialize)]
struct SavePollBody {
    title: String,
    description: String,
    require_name: Option<bool>,
    allow_participant_options: Option<bool>,
    poll_type: String,
    options: Vec<SavePollBodyOption>,
}
pub async fn edit_page_save(mut request: crate::Request) -> tide::Result {
    let body: SavePollBody = request.body_json().await?;
    let id = request.param("poll_id")?;

    let require_name = body.require_name.unwrap_or(false);
    let allow_participant_options = body.allow_participant_options.unwrap_or(false);

    sqlx::query!(
        r#"
        UPDATE polls SET
        title = ?1,
        description = ?2,
        require_name = ?3,
        allow_participant_options = ?4,
        poll_type = ?5
        WHERE id = ?6
        "#,
        body.title,
        body.description,
        require_name,
        allow_participant_options,
        body.poll_type,
        id,
    )
    .execute(&request.state().db)
    .await?;

    let options_before = sqlx::query_as!(
        IdRow,
        r#"
        SELECT id
        FROM options
        WHERE poll_id = ?1
        ORDER BY order_index
        "#,
        id
    )
    .fetch_all(&request.state().db)
    .await?;

    for opt in options_before {
        match body.options.iter().find(|body_opt| body_opt.id == opt.id) {
            None => {
                sqlx::query!(
                    r#"
                    DELETE FROM options
                    WHERE id = ?1 AND poll_id = ?2
                    "#,
                    opt.id,
                    id,
                )
                .execute(&request.state().db)
                .await?;
            }
            _ => {}
        }
    }

    for (index, opt) in body.options.iter().enumerate() {
        let option_index: i64 = index.try_into()?;
        if opt.id < 0 {
            sqlx::query!(
                r#"
                INSERT INTO options (name, order_index, poll_id)
                VALUES (?1, ?2, ?3)
                "#,
                opt.name,
                option_index,
                id,
            )
            .execute(&request.state().db)
            .await?;
        } else {
            sqlx::query!(
                r#"
                UPDATE options SET
                name = ?1,
                order_index = ?2
                WHERE id = ?3 AND poll_id = ?4
                "#,
                opt.name,
                option_index,
                opt.id,
                id,
            )
            .execute(&request.state().db)
            .await?;
        }
    }

    let options = sqlx::query_as!(
        EditPageOptionQueryResult,
        r#"
        SELECT
            id,
            name
        FROM options
        WHERE poll_id = ?1
        ORDER BY order_index
        "#,
        id
    )
    .fetch_all(&request.state().db)
    .await?;

    Ok(EditPageForm {
        id: id.to_string(),
        title: body.title,
        description: body.description,
        require_name: require_name,
        allow_participant_options: allow_participant_options,
        poll_type: body.poll_type,
        options: options
            .iter()
            .map(|o| EditPageOption {
                id: o.id,
                name: o.name.to_owned(),
            })
            .collect(),
    }
    .into())
}

struct Published {
    published: bool,
}
pub async fn edit_page_toggle_publish(mut request: crate::Request) -> tide::Result {
    let id = request.param("poll_id")?;

    sqlx::query!(
        r#"
        UPDATE polls SET
        published = ((published | 1) - (published & 1))
        WHERE id = ?1
        "#,
        id
    )
    .execute(&request.state().db)
    .await?;

    let p = sqlx::query_as!(
        Published,
        r#"
        SELECT published
        FROM polls
        WHERE id = ?1
        "#,
        id
    )
    .fetch_one(&request.state().db)
    .await?;

    Ok(EditPagePublish {
        id: id.to_string(),
        published: p.published,
    }
    .into())
}

struct ResultsPollDetails {
    title: String,
}
#[derive(FromRow)]
struct SubmissionQueryResult {
    score: i64,
    option_id: i64,
    option_name: String,
    order_index: i64,
}
async fn create_results_page(
    poll_id: &str,
    db: &SqlitePool,
    poll: ResultsPollDetails,
) -> Result<ResultsPage> {
    let submissions = sqlx::query_as!(
        SubmissionQueryResult,
        r#"
        SELECT
            COALESCE(s.score, 0) AS "score!",
            o.id as option_id,
            o.name AS option_name,
            o.order_index
        FROM options o
            LEFT OUTER JOIN submissions s
            ON s.option_id = o.id
        WHERE o.poll_id = ?1
        "#,
        poll_id
    )
    .fetch_all(db)
    .await?;
    let option_result_map: HashMap<i64, OptionResult> =
        submissions
            .iter()
            .fold(HashMap::new(), |mut results, submission| {
                let mut result =
                    results
                        .entry(submission.option_id)
                        .or_insert_with(|| OptionResult {
                            id: submission.option_id,
                            name: submission.option_name.clone(),
                            score: 0,
                            order_index: submission.order_index,
                        });
                result.score += submission.score;
                results
            });
    let mut option_results: Vec<OptionResult> =
        option_result_map.into_iter().map(|r| r.1).collect();
    option_results.sort_by_key(|r| r.order_index);
    let largest_score = option_results
        .iter()
        .fold(1, |largest, r| max(largest, r.score));
    Ok(ResultsPage {
        html_title: format!("Results | {}", poll.title),
        title: poll.title,
        option_results: option_results,
        largest_score: largest_score,
    })
}

#[derive(FromRow)]
struct SubmitPollQueryResult {
    title: String,
    require_name: bool,
    allow_participant_options: bool,
    poll_type: String,
    published: bool,
}
#[derive(Deserialize)]
struct OptionScore {
    id: i64,
    score: i64,
}
async fn submit(
    poll_id: &str,
    db: &SqlitePool,
    poll_type: &str, // TODO replace poll_type string here with generic/typed function
    participant_name: Option<String>,
    new_option: Option<String>,
    scores: Vec<OptionScore>,
) -> tide::Result {
    let poll = sqlx::query_as!(
        SubmitPollQueryResult,
        r#"
        SELECT
            title,
            require_name,
            allow_participant_options,
            poll_type,
            published
        FROM polls
        WHERE id = ?1
        "#,
        poll_id
    )
    .fetch_one(db)
    .await?;

    if !poll.published {
        Ok(tide::Response::builder(404).build())
    } else if poll.poll_type != poll_type {
        Ok(tide::Response::builder(400).body("wrong poll_type").build())
    } else if poll.require_name
        && (participant_name == None || participant_name == Some("".to_string()))
    {
        Ok(tide::Response::builder(400)
            .body("name required but not provided")
            .build())
    } else if !poll.allow_participant_options && new_option.is_some() {
        Ok(tide::Response::builder(400)
            .body("participant options not allowed but provided")
            .build())
    } else {
        // TODO handle new_option
        for score in scores {
            sqlx::query!(
                r#"
                INSERT INTO submissions (
                    participant_name,
                    score,
                    option_id,
                    poll_id
                )
                VALUES (?1, ?2, ?3, ?4)
                "#,
                participant_name,
                score.score,
                score.id,
                poll_id
            )
            .execute(db)
            .await?;
        }

        Ok(tide::Response::builder(200)
            .header(
                "Set-Cookie",
                format!("{}=true; HttpOnly", poll_id),
            )
            .body(
                create_results_page(poll_id, db, ResultsPollDetails { title: poll.title })
                    .await?
                    .to_string(),
            )
            .build())
    }
}
#[derive(Deserialize)]
struct SingleSubmission {
    selection: i64,
    new_option: Option<String>,
    participant_name: Option<String>,
}
pub async fn submit_single(mut request: crate::Request) -> tide::Result {
    let body: SingleSubmission = request.body_json().await?;
    let id = request.param("poll_id")?;

    if request.cookie(id).is_some() {
        Ok(tide::Response::builder(400)
            .body("already submitted")
            .build())
    } else {
        if body.selection == -1
            && (body.new_option == None || body.new_option == Some("".to_string()))
        {
            Ok(tide::Response::builder(400).build())
        } else {
            submit(
                id,
                &request.state().db,
                POLL_TYPE_SINGLE,
                body.participant_name,
                body.new_option,
                vec![OptionScore {
                    score: 1,
                    id: body.selection,
                }],
            )
            .await
        }
    }
}

#[derive(Deserialize)]
struct NewOption {
    name: String,
    create: bool,
}
#[derive(Deserialize)]
struct MultiSubmission {
    selections: Vec<i64>,
    new_option: Option<NewOption>,
    participant_name: Option<String>,
}
pub async fn submit_multi(mut request: crate::Request) -> tide::Result {
    let body: MultiSubmission = request.body_json().await?;
    let id = request.param("poll_id")?;

    if request.cookie(id).is_some() {
        Ok(tide::Response::builder(400)
            .body("already submitted")
            .build())
    } else {
        let new_option_str = match body.new_option {
            Some(new_option) => {
                if new_option.create {
                    if new_option.name == "" {
                        None
                    } else {
                        Some(new_option.name)
                    }
                } else {
                    None
                }
            }
            None => None,
        };

        submit(
            id,
            &request.state().db,
            POLL_TYPE_MULTI,
            body.participant_name,
            new_option_str,
            body.selections
                .iter()
                .map(|s| OptionScore { score: 1, id: *s })
                .collect(),
        )
        .await
    }
}

#[derive(Deserialize)]
struct ScoreSubmission {
    scores: Vec<OptionScore>,
    new_option: Option<NewOption>,
    participant_name: Option<String>,
}
pub async fn submit_score(mut request: crate::Request) -> tide::Result {
    let body: ScoreSubmission = request.body_json().await?;
    let id = request.param("poll_id")?;

    if request.cookie(id).is_some() {
        Ok(tide::Response::builder(400)
            .body("already submitted")
            .build())
    } else {
        let new_option_str = match body.new_option {
            Some(new_option) => {
                if new_option.create {
                    if new_option.name == "" {
                        None
                    } else {
                        Some(new_option.name)
                    }
                } else {
                    None
                }
            }
            None => None,
        };

        submit(
            id,
            &request.state().db,
            POLL_TYPE_SCORE,
            body.participant_name,
            new_option_str,
            body.scores,
        )
        .await
    }
}

#[derive(FromRow)]
struct ResultsPollQueryResult {
    title: String,
    published: bool,
}
pub async fn results_page(mut request: crate::Request) -> tide::Result {
    let id = request.param("poll_id")?;
    let poll = sqlx::query_as!(
        ResultsPollQueryResult,
        r#"
        SELECT
            title,
            published
        FROM polls
        WHERE id = ?1
        "#,
        id
    )
    .fetch_one(&request.state().db)
    .await?;
    if !poll.published {
        Ok(tide::Response::builder(404).build())
    } else {
        Ok(create_results_page(
            &id,
            &request.state().db,
            ResultsPollDetails { title: poll.title },
        )
        .await?
        .into())
    }
}
