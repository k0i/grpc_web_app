#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configuration =
        kb::configuration::get_configuration().expect("Failed to read configuration.");
    let addr = format!("127.0.0.1:{}", configuration.application_port).parse()?;
    let _ = kb::run()?.serve(addr).await;
    Ok(())
}
