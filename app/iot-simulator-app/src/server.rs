use leptos::{prelude::ServerFnError, server};

#[server]
pub async fn start_flow() -> Result<(), ServerFnError> {
    println!("ok");

    Ok(())
}
