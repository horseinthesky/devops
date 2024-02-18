use tokio::time::{sleep, Duration};
use std::time::SystemTime;
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
    download().await;

    let image = Image::new();
    save(image).await;
}
