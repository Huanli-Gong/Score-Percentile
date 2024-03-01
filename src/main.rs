use aws_config::load_from_env;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{LambdaEvent, Error as LambdaError, service_fn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use simple_logger::SimpleLogger;

#[derive(Deserialize)]
struct Request {
    score: i32, // Receive a score as request input
}

#[derive(Serialize)]
struct Response {
    percentile: f64, // Return the percentile rank of the score
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    SimpleLogger::new().with_utc_timestamps().init()?;
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    let request: Request = serde_json::from_value(event.payload)?;

    let config = load_from_env().await;
    let client = Client::new(&config);

    let percentile = get_percentile(&client, request.score).await?;

    Ok(json!({ "percentile": percentile }))
}

async fn get_percentile(client: &Client, input_score: i32) -> Result<f64, LambdaError> {
    let table_name = "StudentScores"; // Adjust to your DynamoDB table name for scores

    // Scan the table to retrieve all scores
    let result = client.scan()
        .table_name(table_name)
        .send()
        .await?;

    let items = result.items.unwrap_or_default();

    // Extract scores and calculate the percentile
    let scores: Vec<i32> = items.iter()
        .filter_map(|item| item.get("score").and_then(|val| val.as_n().ok()).and_then(|n| n.parse::<i32>().ok()))
        .collect();

    // Calculate the number of scores higher than the input score
    let higher_scores_count = scores.iter().filter(|&&score| score > input_score).count();
    let total_scores = scores.len();

    // Calculate the top percentile
    let percentile = if total_scores > 0 {
        100.0 * (1.0 - (higher_scores_count as f64 / total_scores as f64))
    } else {
        0.0 // Return 0 if there are no scores
    };

    Ok(percentile)
}
