fn main() {
    let password_min = 254032;
    let password_max = 789860;
    let mut count = 0;
    
    for p in password_min..=password_max {
        let mut m = 10;
        let mut non_decreasing = true;
        let mut has_twins = false;
        
        for _ in 1..6 {
            let d1 = p % m / (m / 10);
            let d2 = p % (m * 10) / m;
            m *= 10;
            if !has_twins && d1 == d2 {
                has_twins = true;
            }
            if d2 > d1 {
                non_decreasing = false;
                break;
            }
        }
        
        if non_decreasing && has_twins {
            println!("{}", p);
            count += 1;
        }
    }
    
    println!("{}", count);
}