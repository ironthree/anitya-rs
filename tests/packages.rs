use std::time::Duration;

use anitya::v2::PackageQuery;
use anitya::ClientBuilder;

#[tokio::test]
async fn package_query() {
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    let client = ClientBuilder::new("https://release-monitoring.org")
        .with_delay(Duration::from_millis(1000))
        .build()
        .unwrap();

    let query = PackageQuery::new();
    let _result = client.paginated_request(&query).await.unwrap();
}
