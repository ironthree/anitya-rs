use anitya::v2::PackageQuery;
use anitya::AnityaClientBuilder;
use anitya::PaginatedRequest;

#[tokio::test]
async fn query() {
    let client = AnityaClientBuilder::new("https://release-monitoring.org")
        .build()
        .unwrap();
    let request = PackageQuery::new().page_request(1);
    let _result = client.request(&request).await.unwrap();
}
