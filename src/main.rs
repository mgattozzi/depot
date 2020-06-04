use std::error::Error;

#[tokio::main]
async fn main() {
  if let Err(e) = run().await {
    eprintln!("{}", e);
  }
}

async fn run() -> Result<(), Box<dyn Error>> {
  Ok(())
}
