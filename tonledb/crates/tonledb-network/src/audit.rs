use serde::Serialize;
use std::{fs::OpenOptions, io::Write, sync::Mutex};
use once_cell::sync::Lazy;

#[derive(Serialize)]
pub struct AuditEvent<'a> { pub ts:&'a str, pub who:&'a str, pub action:&'a str, pub resource:&'a str, pub result:&'a str }

static AUDIT: Lazy<Mutex<std::fs::File>> = Lazy::new(|| {
    std::fs::create_dir_all("./logs").ok();
    Mutex::new(OpenOptions::new().create(true).append(true).open("./logs/audit.jsonl").unwrap())
});
pub fn log(ev: &AuditEvent) {
    let s = serde_json::to_string(ev).unwrap();
    let mut f = AUDIT.lock().unwrap();
    let _ = writeln!(f, "{}", s);
}
