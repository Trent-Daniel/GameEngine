use std::time::SystemTime;

pub fn now() -> String
{
    // TODO: Fix this unwrap() to have a match statement
    let s: u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let t: u64 = s % 86400;
    let seconds = t % 60;
    let minutes = (t / 60) % 60;
    let hours = t / 3600;
    let z: i64 = s as i64 / 86400 + 719468;
    let era: i64;
    if z >= 0
    {
        era = z / 146097;
    }
    else
    {
        era = (z - 146096) / 146097;
    }
    let doe: u64 = (z - era * 146097) as u64;
    let yoe: u64 = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut year: i64 = yoe as i64 + era * 400;
    let doy: u64 = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp: u64 = (5 * doy + 2) / 153;
    let day: u64 = doy - (153 * mp + 2) / 5 + 1;
    let month: u64;
    if mp < 10
    {
        month = mp + 3;
    }
    else
    {
        month = mp - 9;
    }
    if month <= 2
    {
        year += 1;
    }

    let timestamp = year.to_string()
        + "-"
        + &(month.to_string())
        + "-"
        + &(day.to_string())
        + "::"
        + &(hours.to_string())
        + ":"
        + &(minutes.to_string())
        + ":"
        + &(seconds.to_string());

    return timestamp;
}
