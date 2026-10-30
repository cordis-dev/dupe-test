pub fn clamp_percent(value: i32) -> i32 {
    if value < 0 {
        return 0;
    }
    if value > 100 {
        return 100;
    }
    value
}
