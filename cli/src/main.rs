pub mod cmd;
pub mod output;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello from CLI");
    Ok(())
}