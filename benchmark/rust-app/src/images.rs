use std::time::{Instant, SystemTime};
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug)]
struct Image {
    uuid: Uuid,
    modified: SystemTime,
}

impl Image {
    fn new() -> Self {
        Image {
            uuid: Uuid::new_v4(),
            modified: SystemTime::now(),
        }
    }
}

#[tracing::instrument]
async fn download() {
    sleep(Duration::from_millis(5)).await;
}

#[tracing::instrument]
async fn save(_image: Image) {
    sleep(Duration::from_millis(2)).await;
}

#[tracing::instrument]
pub async fn get_images() {
    let start = Instant::now();
    download().await;

    metrics::histogram!(
        "myapp_request_duration_seconds",
        &[("operation", "s3")],
    )
    .record(start.elapsed());

    let image = Image::new();

    let start = Instant::now();
    save(image).await;

    metrics::histogram!(
        "myapp_request_duration_seconds",
        &[("operation", "db")],
    )
    .record(start.elapsed());
}
