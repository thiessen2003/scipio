use std::env;
use tracing_test::traced_test;
use anyhow::Result;
use rstest::rstest;

use super::fixtures::{context, AsyncTestContext};
use super::Customer;
use crate::base_data::records::{GetRecordQueryBuilder, ListRecordsQueryBuilder};

#[cfg(feature = "integration")]
#[rstest]
#[traced_test]
#[tokio::test]
pub async fn test_list_records(context: AsyncTestContext) -> Result<()> {
    let mut cleanup = context.cleanup.lock().await;
    cleanup.push(Box::new(|| {
        Box::pin(async move {
            println!("Cleaning up test_list_records");
            Ok(())
        })
    }));

    let base = env::var("TEST_AIRTABLE_API_BASE").expect("missing TEST_AIRTABLE_BASE variable");
    let table = env::var("TEST_AIRTABLE_API_TABLE").expect("missing TEST_AIRTABLE_TABLE variable");
    let view = env::var("TEST_AIRTABLE_API_VIEW").expect("missing TEST_AIRTABLE_VIEW variable");

    let query = ListRecordsQueryBuilder::default()
        .fields(Customer::field_names().iter().map(ToString::to_string).collect::<Vec<_>>())
        .view(view)
        .build()?;

    let res = context.airtable.list_records::<Customer>(&base, &table, Some(&query)).await?;

    dbg!(&res.records);

    Ok(())
}

#[cfg(feature = "integration")]
#[rstest]
#[traced_test]
#[tokio::test]
pub async fn test_get_record(context: AsyncTestContext) -> Result<()> {
    use anyhow::bail;

    let mut cleanup = context.cleanup.lock().await;
    cleanup.push(Box::new(|| {
        Box::pin(async move {
            tracing::info!("Cleaning up test_get_record");
            // bail!("test_get_record failed");
            Ok(())
        })
    }));

    let base = env::var("TEST_AIRTABLE_API_BASE").expect("missing TEST_AIRTABLE_BASE variable");
    let table = env::var("TEST_AIRTABLE_API_TABLE").expect("missing TEST_AIRTABLE_TABLE variable");
    let record_id =
        env::var("TEST_AIRTABLE_API_RECORD_ID").expect("missing TEST_AIRTABLE_RECORD_ID variable");

    let query = GetRecordQueryBuilder::default().build()?;

    let res =
        context.airtable.get_record::<Customer>(&base, &table, &record_id, Some(&query)).await?;

    dbg!(&res);

    Ok(())
}

#[cfg(feature = "integration")]
#[rstest]
#[traced_test]
#[tokio::test]
pub async fn test_update_record(context: AsyncTestContext) -> Result<()> {
    let mut cleanup = context.cleanup.lock().await;
    let airtable = context.airtable.clone();
    cleanup.push(Box::new(move || {
        let airtable = airtable.clone();
        Box::pin(async move {
            tracing::info!("Cleaning up test_get_record");
            Ok(())
        })
    }));

    Ok(())
}

#[cfg(feature = "integration")]
#[rstest]
#[traced_test]
#[tokio::test]
pub async fn test_update_record(context: AsyncTestContext) -> Result<()> {
    let mut cleanup = context.cleanup.lock().await;
    let airtable = context.airtable.clone();
    
    cleanup.push(Box::new(move || {
        let airtable = airtable.clone();
        Box::pin(async move {
            tracing::info!("Cleaning up test_update_record");
            Ok(())
        })
    }));

    let base = env::var("TEST_AIRTABLE_API_BASE").expect("missing TEST_AIRTABLE_BASE variable");
    let table = env::var("TEST_AIRTABLE_API_TABLE").expect("missing TEST_AIRTABLE_TABLE variable");
    let record_id = env::var("TEST_AIRTABLE_API_RECORD_ID")
        .expect("missing TEST_AIRTABLE_RECORD_ID variable");

    // Create test data
    let update_data = UpdateRecordPayload {
        fields: Customer {
            // Add your test customer fields here
            name: "Updated Test Customer".to_string(),
            // ... other fields
        },
    };

    // Test PATCH update
    let patch_result = context
        .airtable
        .update_record(&base, &table, &record_id, update_data.clone(), UpdateMethod::Patch)
        .await?;
    
    assert!(patch_result.id.len() > 0);
    assert_eq!(patch_result.fields.name, "Updated Test Customer");

    // Test PUT update
    let put_result = context
        .airtable
        .update_record(&base, &table, &record_id, update_data, UpdateMethod::Put)
        .await?;
    
    assert!(put_result.id.len() > 0);
    assert_eq!(put_result.fields.name, "Updated Test Customer");

    Ok(())
}

#[cfg(feature = "integration")]
#[rstest]
#[traced_test]
#[tokio::test]
pub async fn test_update_multiple_records(context: AsyncTestContext) -> Result<()> {
    let mut cleanup = context.cleanup.lock().await;
    let airtable = context.airtable.clone();
    
    cleanup.push(Box::new(move || {
        let airtable = airtable.clone();
        Box::pin(async move {
            tracing::info!("Cleaning up test_update_multiple_records");
            Ok(())
        })
    }));

    let base = env::var("TEST_AIRTABLE_API_BASE").expect("missing TEST_AIRTABLE_BASE variable");
    let table = env::var("TEST_AIRTABLE_API_TABLE").expect("missing TEST_AIRTABLE_TABLE variable");

    // Create test data for multiple records
    let records = vec![
        UpdateRecordPayload {
            fields: Customer {
                name: "Batch Update Customer 1".to_string(),
                // ... other fields
            },
        },
        UpdateRecordPayload {
            fields: Customer {
                name: "Batch Update Customer 2".to_string(),
                // ... other fields
            },
        },
    ];

    let update_data = UpdateMultipleRecordsPayload {
        records,
        typecast: Some(false),
    };

    // Test PATCH update for multiple records
    let patch_result = context
        .airtable
        .update_multiple_records(&base, &table, update_data.clone(), UpdateMethod::Patch)
        .await?;
    
    assert_eq!(patch_result.records.len(), 2);
    assert!(patch_result.records[0].id.len() > 0);
    assert!(patch_result.records[1].id.len() > 0);
    assert_eq!(patch_result.records[0].fields.name, "Batch Update Customer 1");
    assert_eq!(patch_result.records[1].fields.name, "Batch Update Customer 2");

    // Test PUT update for multiple records
    let put_result = context
        .airtable
        .update_multiple_records(&base, &table, update_data, UpdateMethod::Put)
        .await?;
    
    assert_eq!(put_result.records.len(), 2);
    assert!(put_result.records[0].id.len() > 0);
    assert!(put_result.records[1].id.len() > 0);
    assert_eq!(put_result.records[0].fields.name, "Batch Update Customer 1");
    assert_eq!(put_result.records[1].fields.name, "Batch Update Customer 2");

    Ok(())
}

// Helper structs that you'll need to define if not already present
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRecordPayload<T> {
    pub fields: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMultipleRecordsPayload<T> {
    pub records: Vec<UpdateRecordPayload<T>>,
    pub typecast: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateMethod {
    Patch,
    Put,
}