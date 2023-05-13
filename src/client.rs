use serde_json::json;
use std::{collections::HashMap, str::FromStr, sync::Arc, time::Duration};
use temporal_client::WorkflowOptions;
use temporal_sdk::sdk_client_options;
use temporal_sdk_core::{protos::temporal::api::common::v1::Payload, Url, WorkflowClientTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;

    let client = server_options.connect("default", None, None).await?;

    let mut metadata = HashMap::new();
    metadata.insert("encoding".to_owned(), "json/plain".as_bytes().to_vec());

    let payload1 = Payload {
        metadata: metadata.to_owned(),
        data: json!("first").to_string().into(),
    };

    let payload2 = Payload {
        metadata: metadata.to_owned(),
        data: json!("second").to_string().into(),
    };

    let _handle = client
        .start_workflow(
            vec![payload1, payload2],
            "hello-world".to_owned(),
            "workflow-id".to_owned(),
            "example".to_owned(),
            None,
            WorkflowOptions {
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}
