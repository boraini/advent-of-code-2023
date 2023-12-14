pub fn find_distance(time: &i64, hold_time: &i64) -> i64 {
    let race_time = time - hold_time;

    hold_time * race_time
}