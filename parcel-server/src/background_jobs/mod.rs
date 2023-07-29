mod delete_expired_sessions;

use std::{sync::Arc, time::Duration};

use futures_util::FutureExt;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::data::database::Database;

pub async fn create_scheduler(database: Arc<Database>) -> Result<JobScheduler, JobSchedulerError> {
    let scheduler = JobScheduler::new().await?;

    scheduler
        .add(Job::new_repeated_async(
            Duration::from_secs(60 * 10),
            move |_uuid, _lock| {
                let db_clone = database.clone();
                async move {
                    log_result(
                        "DeleteExpiredSessions",
                        delete_expired_sessions::delete_expired_sessions(db_clone).await,
                    );
                }
                .boxed()
            },
        )?)
        .await?;

    Ok(scheduler)
}

/// Takes a result and logs an error if it failed
fn log_result<T, E>(job_name: &str, result: Result<T, E>)
where
    E: std::fmt::Display + std::fmt::Debug,
{
    if let Err(err) = &result {
        log::error!("{job_name} failed: {err}");
    }
}
