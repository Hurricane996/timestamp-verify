use chrono::Utc;
use timestamp_verify::sign;

fn main() {
    let dt = Utc::now();

    let signature = sign(dt.timestamp());

    println!("{}", dt.format("%B%e, %Y %H:%M:%S"));

    println!("Timestamp: {}, MAC: {}", dt.timestamp(), signature);
}
