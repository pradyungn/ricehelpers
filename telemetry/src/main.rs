use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let  sincetime = since_the_epoch.as_secs();
    let currhour = (sincetime/3600%24 + 17) % 24;
    let currmin = sincetime/60%60;
}
