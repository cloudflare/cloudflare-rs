use crate::{AsyncClient, ResultExt};
use cloudflare::endpoints::workerskv::create_namespace::{CreateNamespace, CreateNamespaceParams};
use cloudflare::endpoints::workerskv::delete_bulk::DeleteBulk;
use cloudflare::endpoints::workerskv::delete_key::DeleteKey;
use cloudflare::endpoints::workerskv::list_namespace_keys::ListNamespaceKeys;
use cloudflare::endpoints::workerskv::list_namespaces::ListNamespaces;
use cloudflare::endpoints::workerskv::read_key::ReadKey;
use cloudflare::endpoints::workerskv::read_key_metadata::ReadKeyMetadata;
use cloudflare::endpoints::workerskv::remove_namespace::RemoveNamespace;
use cloudflare::endpoints::workerskv::rename_namespace::{RenameNamespace, RenameNamespaceParams};
use cloudflare::endpoints::workerskv::write_bulk::{KeyValuePair, WriteBulk};
use cloudflare::endpoints::workerskv::write_key::WriteKey;
use cloudflare::endpoints::workerskv::write_key::{WriteKeyBody, WriteKeyBodyMetadata};
use cloudflare::endpoints::workerskv::{Key, WorkersKvBulkResult, WorkersKvNamespace};
use cloudflare::framework::async_api::Client;
use cloudflare::framework::response::{ApiFailure, ApiResponse, ApiSuccess};
use rand;
use rand::Rng;
use serde_json::json;

async fn read_key(
    client: &Client,
    account_id: &str,
    namespace_id: &str,
    key: &str,
) -> ApiResponse<Vec<u8>> {
    let endpoint = ReadKey {
        account_identifier: account_id,
        namespace_identifier: namespace_id,
        key,
    };

    client.request(&endpoint).await
}

async fn create_namespace(
    client: &Client,
    account_id: &str,
    title: &str,
) -> ApiResponse<ApiSuccess<WorkersKvNamespace>> {
    let endpoint = CreateNamespace {
        account_identifier: account_id,
        params: CreateNamespaceParams {
            title: title.into(),
        },
    };

    client.request(&endpoint).await
}

async fn delete_bulk(
    client: &Client,
    account_id: &str,
    namespace_id: &str,
    keys: Vec<&str>,
) -> ApiResponse<ApiSuccess<WorkersKvBulkResult>> {
    let endpoint = DeleteBulk {
        account_identifier: account_id,
        namespace_identifier: namespace_id,
        bulk_keys: keys.into_iter().map(|k| k.into()).collect(),
    };

    client.request(&endpoint).await
}

async fn delete_key(
    client: &Client,
    account_id: &str,
    namespace_id: &str,
    key: &str,
) -> ApiResponse<ApiSuccess<()>> {
    let endpoint = DeleteKey {
        account_identifier: account_id,
        namespace_identifier: namespace_id,
        key,
    };

    client.request(&endpoint).await
}

async fn list_namespace_keys(
    client: &Client,
    account_id: &str,
    namespace_id: &str,
) -> ApiResponse<ApiSuccess<Vec<Key>>> {
    let endpoint = ListNamespaceKeys {
        account_identifier: account_id,
        namespace_identifier: namespace_id,
        params: Default::default(),
    };

    client.request(&endpoint).await
}

async fn list_namespaces(
    client: &Client,
    account_id: &str,
) -> ApiResponse<ApiSuccess<Vec<WorkersKvNamespace>>> {
    let endpoint = ListNamespaces {
        account_identifier: account_id,
        params: Default::default(),
    };

    client.request(&endpoint).await
}

async fn read_key_metadata(
    client: &Client,
    account_id: &str,
    namespace_id: &str,
    key: &str,
) -> ApiResponse<ApiSuccess<Option<serde_json::Value>>> {
    let endpoint = ReadKeyMetadata {
        account_identifier: account_id,
        namespace_identifier: namespace_id,
        key,
    };

    client.request(&endpoint).await
}

async fn remove_namespace(
    client: &Client,
    account_id: &str,
    namespace_id: &str,
) -> ApiResponse<ApiSuccess<()>> {
    let endpoint = RemoveNamespace {
        account_identifier: account_id,
        namespace_identifier: namespace_id,
    };

    client.request(&endpoint).await
}

async fn rename_namespace(
    client: &Client,
    account_id: &str,
    namespace_id: &str,
    title: &str,
) -> ApiResponse<ApiSuccess<()>> {
    let endpoint = RenameNamespace {
        account_identifier: account_id,
        namespace_identifier: namespace_id,
        params: RenameNamespaceParams {
            title: title.into(),
        },
    };

    client.request(&endpoint).await
}

async fn write_bulk(
    client: &Client,
    account_id: &str,
    namespace_id: &str,
    key_value_pairs: Vec<(&str, &str)>,
) -> ApiResponse<ApiSuccess<WorkersKvBulkResult>> {
    let endpoint = WriteBulk {
        account_identifier: account_id,
        namespace_identifier: namespace_id,
        bulk_key_value_pairs: key_value_pairs
            .into_iter()
            .map(|(k, v)| KeyValuePair {
                key: k.into(),
                value: v.into(),
                expiration: None,
                expiration_ttl: None,
                base64: None,
            })
            .collect(),
    };

    client.request(&endpoint).await
}

async fn write_key_metadata(
    client: &Client,
    account_id: &str,
    namespace_id: &str,
    key: &str,
    value: Vec<u8>,
    metadata: Option<serde_json::Value>,
) -> ApiResponse<ApiSuccess<()>> {
    let endpoint = WriteKey {
        account_identifier: account_id,
        namespace_identifier: namespace_id,
        key,
        params: Default::default(),
        body: if let Some(metadata) = metadata {
            WriteKeyBody::Metadata(WriteKeyBodyMetadata { value, metadata })
        } else {
            WriteKeyBody::Value(value)
        },
    };

    client.request(&endpoint).await
}

pub async fn test_kv(client: &AsyncClient, account_id: &str) -> anyhow::Result<()> {
    //region Create a new namespace
    println!("Creating a new namespace...");

    let title: String = get_random_title();
    let result = create_namespace(client, account_id, &title)
        .await
        .log_err(|e| println!("Error while creating namespace: {e}"))?
        .result;
    assert_eq!(result.title, title);
    assert_eq!(result.supports_url_encoding, Some(true));
    //endregion

    let namespace_id = result.id;
    let namespace_id = namespace_id.as_str();
    let key = "key";

    //region List all namespaces
    println!("Listing all namespaces...");

    let result = list_namespaces(client, account_id)
        .await
        .log_err(|e| println!("Error while listing namespaces: {e}"))?
        .result;
    assert!(result.contains(&WorkersKvNamespace {
        id: namespace_id.to_string(),
        title: title.clone(),
        supports_url_encoding: Some(true),
    }));
    //endregion

    //region Write a key-value pair
    println!("Writing a key-value pair...");

    write_key_metadata(
        client,
        account_id,
        namespace_id,
        key,
        b"value".to_vec(),
        Some(serde_json::to_value(json!({"metadata": true})).unwrap()),
    )
    .await
    .log_err(|e| println!("Error while writing key: {e}"))?;
    //endregion

    //region Read a key-value pair
    println!("Reading a key-value pair...");

    let result = read_key(client, account_id, namespace_id, key)
        .await
        .log_err(|e| println!("Error while reading key: {e}"))?;
    assert_eq!(result, b"value");
    //endregion

    //region Write multiple key-value pairs
    println!("Writing multiple key-value pairs...");

    let key_value_pairs = vec![("debug", "test"), ("debug2", "test2")];

    let result = write_bulk(client, account_id, namespace_id, key_value_pairs.clone())
        .await
        .log_err(|e| println!("Error while writing bulk: {e}"))?
        .result;
    assert_eq!(result.successful_key_count.unwrap(), 2);
    assert_eq!(result.unsuccessful_keys, Some(vec![]));
    let result = read_key(
        client,
        account_id,
        namespace_id,
        key_value_pairs.clone()[0].0,
    )
    .await
    .log_err(|e| println!("Error while reading key: {e}"))?;
    assert_eq!(result, b"test");
    let result = read_key(client, account_id, namespace_id, key_value_pairs[1].0)
        .await
        .log_err(|e| println!("Error while reading key: {e}"))?;
    assert_eq!(result, b"test2");
    //endregion

    //region Read a key-value pair's metadata
    println!("Reading a key-value pair's metadata...");

    let result = read_key_metadata(client, account_id, namespace_id, key)
        .await
        .log_err(|e| println!("Error while reading key metadata: {e}"))?
        .result;
    assert_eq!(result, Some(json!({"metadata": true})));
    //endregion

    //region List all keys in a namespace
    println!("Listing all keys in a namespace...");

    let result = list_namespace_keys(client, account_id, namespace_id)
        .await
        .log_err(|e| println!("Error while listing namespace keys: {e}"))?
        .result;
    assert_eq!(result.len(), 3);
    assert!(result.contains(&Key {
        name: key.to_string(),
        expiration: None,
        metadata: Some(json!({"metadata": true})),
    }));
    assert!(result.contains(&Key {
        name: key_value_pairs[0].0.to_string(),
        expiration: None,
        metadata: None,
    }));
    assert!(result.contains(&Key {
        name: key_value_pairs[1].0.to_string(),
        expiration: None,
        metadata: None,
    }));
    //endregion

    //region Delete a key-value pair
    println!("Deleting a key-value pair...");

    delete_key(client, account_id, namespace_id, key)
        .await
        .log_err(|e| {
            println!("Error while deleting key: {e}");
        })?;
    let result = list_namespace_keys(client, account_id, namespace_id)
        .await
        .log_err(|e| println!("Error while listing namespace keys: {e}"))?
        .result;
    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|k| k.name != key));
    assert!(result.contains(&Key {
        name: key_value_pairs[0].0.to_string(),
        expiration: None,
        metadata: None,
    }));
    assert!(result.contains(&Key {
        name: key_value_pairs[1].0.to_string(),
        expiration: None,
        metadata: None,
    }));
    //endregion

    //region Delete multiple key-value pairs
    println!("Deleting multiple key-value pairs...");

    let keys = vec!["debug", "debug2"];
    delete_bulk(client, account_id, namespace_id, keys)
        .await
        .log_err(|e| {
            println!("Error while deleting bulk: {e}");
        })?;
    let result = list_namespace_keys(client, account_id, namespace_id)
        .await
        .log_err(|e| println!("Error while listing namespace keys: {e}"))?
        .result;
    assert_eq!(result.len(), 0);
    //endregion

    //region Rename a namespace
    println!("Renaming a namespace...");

    let new_title = get_random_title();

    rename_namespace(client, account_id, namespace_id, new_title.as_str())
        .await
        .log_err(|e| {
            println!("Error while renaming namespace: {e}");
        })?;
    let result = list_namespaces(client, account_id)
        .await
        .log_err(|e| println!("Error while listing namespaces: {e}"))?
        .result;
    assert!(result.iter().all(|n| n.title != title));
    assert!(result.contains(&WorkersKvNamespace {
        id: namespace_id.to_string(),
        title: new_title,
        supports_url_encoding: Some(true),
    }));
    //endregion

    //region Remove a namespace
    println!("Removing a namespace...");

    remove_namespace(client, account_id, namespace_id)
        .await
        .log_err(|e| {
            println!("Error while removing namespace: {e}");
        })?;
    let result = list_namespaces(client, account_id)
        .await
        .log_err(|e| println!("Error while listing namespaces: {e}"))?
        .result;
    assert!(result.iter().all(|n| n.title != "test_renamed"));
    //endregion

    //region Set a key on a non-existing namespace and check for error handling
    println!("Error handling check...");

    let result = write_key_metadata(
        client,
        account_id,
        namespace_id,
        key,
        b"value".to_vec(),
        None,
    )
    .await
    .expect_err("Error while checking error handling");
    match result {
        ApiFailure::Error(status, errors) => {
            assert_eq!(status, 404);
            assert_eq!(errors.errors.len(), 1);
            assert_eq!(errors.errors[0].code, 10013);
        }
        ApiFailure::Invalid(e) => {
            panic!("Unexpected error: {e}");
        }
    }
    //endregion

    println!("All KV tests passed!");

    Ok(())
}

fn get_random_title() -> String {
    let title: String = rand::thread_rng()
        // Generate a random string of length 10 from characters 'A' to 'Z'
        .sample_iter(&rand::distributions::Uniform::new(
            char::from(65),
            char::from(90),
        ))
        .take(10)
        .map(char::from)
        .collect();
    title
}
