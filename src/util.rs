use time::Weekday;

pub fn list_weekdays(day: Weekday) -> Vec<Weekday> {
    let mut today = day;
    let mut ret = Vec::new();
    for _ in 0..7 {
        ret.push(today);
        today = today.next();
    }
    ret
}
