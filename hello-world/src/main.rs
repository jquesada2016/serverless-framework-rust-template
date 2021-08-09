use lambda_runtime::{run, Error, handler_fn, Context};
use serde::Deserialize;

#[derive(Deserialize)]
struct Event {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    run(handler_fn(handler)).await?;

    Ok(())
}

async fn handler(e: Event, ctx: Context) -> Result<(), Error> {
    Ok(())
}