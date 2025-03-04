use crate::AsyncClient;

pub async fn test_lb_pool(
    api_client: &AsyncClient,
    account_identifier: &str,
) -> anyhow::Result<()> {
    use crate::ResultExt;
    use cloudflare::endpoints::load_balancing::*;
    use std::net::{IpAddr, Ipv4Addr};

    // Create a pool
    let origins = vec![
        Origin {
            name: "test-origin".to_owned(),
            address: IpAddr::V4(Ipv4Addr::new(152, 122, 3, 1)),
            enabled: true,
            weight: 1.0,
        },
        Origin {
            name: "test-origin-2".to_owned(),
            address: IpAddr::V4(Ipv4Addr::new(152, 122, 3, 2)),
            enabled: true,
            weight: 1.0,
        },
    ];
    let pool = api_client
        .request(&create_pool::CreatePool {
            account_identifier,
            params: create_pool::Params {
                name: "test-pool",
                optional_params: Some(create_pool::OptionalParams {
                    description: Some("test description"),
                    enabled: Some(true),
                    minimum_origins: Some(2),
                    monitor: Some("9004c07f1c0f33255410e45590251cf4"),
                    notification_email: Some("test@example.com"),
                }),
                origins: &origins,
            },
        })
        .await
        .log_err(|e| println!("Error in CreatePool: {e}"))?
        .result;

    // Get the details, but wait until after we delete the pool to validate it.
    let pool_details = api_client
        .request(&pool_details::PoolDetails {
            account_identifier,
            identifier: &pool.id,
        })
        .await
        .log_err(|e| println!("Error in PoolDetails: {e}"));

    // Delete the pool
    let _ = api_client
        .request(&delete_pool::DeletePool {
            account_identifier,
            identifier: &pool.id,
        })
        .await
        .log_err(|e| println!("Error in DeletePool: {e}"))?;

    // Validate the pool we got was the same as the pool we sent
    let pool_details = pool_details?.result;
    assert_eq!(pool, pool_details);

    Ok(())
}
