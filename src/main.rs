use std::{str::FromStr, sync::Arc, time::Duration};
use temporal_sdk::{sdk_client_options, ActContext, ActivityOptions, WfContext, Worker};
use temporal_sdk_core::{init_worker, CoreRuntime, Url};
use temporal_sdk_core_api::{telemetry::TelemetryOptionsBuilder, worker::WorkerConfigBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;

    let client = server_options.connect("default", None, None).await?;

    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("hello-world")
        .worker_build_id("core-worker")
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "hello-world");

    worker.register_activity("first", |_ctx: ActContext, payload: String| async move {
        println!("first activity: {}", payload);
        Ok(format!("We did the first activity: {}", payload))
    });

    worker.register_activity("second", |_ctx: ActContext, payload: String| async move {
        println!("second activity: {}", payload);
        Ok(format!("We did the second activity: {}", payload))
    });

    worker.register_wf("example", |ctx: WfContext| async move {
        println!("Inside workflow");
        let args = ctx.get_args().to_owned();
        if let Some(payload) = args.first() {
            ctx.activity(ActivityOptions {
                activity_type: "first".to_string(),
                start_to_close_timeout: Some(Duration::from_secs(10)),
                input: payload.to_owned(),
                ..Default::default()
            })
            .await;
        }

        if let Some(payload) = args.get(1) {
            ctx.activity(ActivityOptions {
                activity_type: "second".to_string(),
                start_to_close_timeout: Some(Duration::from_secs(10)),
                input: payload.to_owned(),
                ..Default::default()
            })
            .await;
        }
        Ok(temporal_sdk::WfExitValue::Normal("exit value"))
    });

    worker.run().await?;

    Ok(())
}
