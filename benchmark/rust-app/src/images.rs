use tokio::time::{sleep, Duration};

async fn download() {
    sleep(Duration::from_millis(5)).await;
}

async fn save() {
    sleep(Duration::from_millis(2)).await;
}

pub async fn get_images() {
    download().await;
    save().await;
}
