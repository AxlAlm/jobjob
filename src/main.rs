mod command;

#[tokio::main]
async fn main() {
    let raw_command = "ls";
    let output = command::run_raw_command(raw_command).await.unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);
}
