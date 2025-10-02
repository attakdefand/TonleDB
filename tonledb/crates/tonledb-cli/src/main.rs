use clap::{Parser, Subcommand};
use serde::Serialize;
use chrono::Local;


#[derive(Parser, Debug)]
#[command(name="tonledb", version, about="TonleDB CLI")]
struct Args {
#[arg(long, default_value = "http://127.0.0.1:8383")] endpoint: String,
#[command(subcommand)] cmd: Cmd,
}


#[derive(Subcommand, Debug)]
enum Cmd { Sql { query: String }, Init { #[arg(long, default_value = "./tonledb.wal")] wal: String }, Snapshot { #[arg(long, default_value_t = String::new())] out: String } }


#[derive(Serialize)]
struct SqlBody { sql: String }


#[tokio::main]
async fn main() -> anyhow::Result<()> {
let args = Args::parse();
match args.cmd {
Cmd::Sql { query } => do_sql(&args.endpoint, &query).await?,
Cmd::Init { wal } => { std::fs::File::create(&wal)?; println!("Initialized WAL at {}", wal); },
Cmd::Snapshot { out } => { let path = if out.is_empty() { format!("snap-{}.snap", Local::now().format("%Y%m%d-%H%M%S")) } else { out }; std::fs::write(&path, b"demo snapshot\n")?; println!("Wrote {}", path); },
}
Ok(())
}


async fn do_sql(ep: &str, sql: &str) -> anyhow::Result<()> {
let url = format!("{}/sql", ep);
let body = SqlBody { sql: sql.to_string() };
let res: serde_json::Value = reqwest::Client::new().post(url).json(&body).send().await?.json().await?;
println!("{}", serde_json::to_string_pretty(&res)?);
Ok(())
}