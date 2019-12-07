fn check_password(p: i32) -> bool {
    let mut m = 10;
    let mut repeat_count = 1;
    let mut has_twins = false;
    let mut non_decreasing = true;
    
    for _ in 1..6 {
        let d1 = p % m / (m / 10);
        let d2 = p % (m * 10) / m;
        m *= 10;
        if d1 == d2 {
            repeat_count += 1;
        } else {
            if repeat_count == 2 {
                has_twins = true;
            }
            repeat_count = 1;
        }
        if d2 > d1 {
            non_decreasing = false;
            break;
        }
    }
    
    if repeat_count == 2 {
        has_twins = true;
    }
    
    return non_decreasing && has_twins;
}

fn main() {    
    let password_min = 254032;
    let password_max = 789860;
    println!("{}", (password_min..password_max).filter(|p| check_password(*p)).count());
}