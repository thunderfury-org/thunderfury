use chrono::Utc;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use tracing::error;

use super::{delete, param::PushMessageParam, status};
use crate::{
    common::{
        error::{Error, Result},
        state::AppState,
    },
    entity::task,
    library,
};

mod telegram;

const MAX_RETRY_COUNT: u32 = 5;

pub async fn find_message_tasks_not_done(db: &DatabaseConnection) -> Result<Vec<task::Model>> {
    Ok(task::Entity::find()
        .filter(task::Column::Action.eq(task::Action::PushMessage))
        .filter(task::Column::Status.is_in([task::Status::Queued, task::Status::Running]))
        .filter(
            Condition::any()
                .add(task::Column::NextRetryTime.is_null())
                .add(task::Column::NextRetryTime.lte(Utc::now())),
        )
        .order_by_asc(task::Column::Id)
        .all(db)
        .await?)
}

pub async fn run_tasks(state: &AppState, tasks: Vec<task::Model>) {
    for t in &tasks {
        let message: PushMessageParam = t.deserialize_param().unwrap();

        match run_one_task(state, t, &message).await {
            Ok(_) => {}
            Err(e) => {
                error!(task_id = t.id, "run message task error: {}", e);
                break;
            }
        }
    }
}

async fn run_one_task(state: &AppState, t: &task::Model, message: &PushMessageParam) -> Result<()> {
    let mut resend = false;

    match t.status {
        task::Status::Queued => {
            status::update_status_to_running(&state.db, t.id, "").await?;
        }
        task::Status::Running => {
            // may be crashed or killed when send message
            // need retry sending message
            resend = true;
        }
        task::Status::Done | task::Status::Failed => {
            error!(task_id = t.id, "task alread done or failed, shoud not go to here");
            return Ok(());
        }
    }

    let msg = format_message(state, message, resend).await?;

    match telegram::send_message(state, msg.as_str()).await {
        Ok(_) => status::update_status_to_done(&state.db, t.id).await,
        Err(Error::NotFound(_, _)) => {
            // channel not config, delete this task
            delete::delete_task(&state.db, t.id).await
        }
        Err(e) => {
            let retry_count = t.retry_count.unwrap_or(1);
            if retry_count > MAX_RETRY_COUNT {
                status::update_status_to_failed(&state.db, t.id, e.to_string().as_str()).await?;
            } else {
                status::update_status_to_retry(&state.db, t.id, retry_count, e.to_string().as_str()).await?;
            }
            Err(e)
        }
    }
}

async fn format_message(state: &AppState, message: &PushMessageParam, resend: bool) -> Result<String> {
    let msg = match message {
        PushMessageParam::EpisodeDownloaded {
            tv_id,
            season_number,
            episode_number,
        } => {
            let tv_info = library::tv::get_or_fail(&state.db, *tv_id).await?;
            format!(
                "{} ({}) 第 {} 季 第 {} 集下载完成",
                tv_info.name, tv_info.year, season_number, episode_number
            )
        }
        PushMessageParam::MovieDownloaded { movie_id } => {
            let movie_info = library::movie::get_or_fail(&state.db, *movie_id).await?;
            format!("{} ({}) 下载完成", movie_info.name, movie_info.year)
        }
    };

    if resend {
        Ok(format!("{}\n\n重试发送消息, 如已收到请忽略", msg))
    } else {
        Ok(msg)
    }
}
