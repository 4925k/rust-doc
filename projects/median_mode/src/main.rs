fn main() {
    let nums = vec![2, 33, 51, 43, 6, 234, 51, 11];

    let (median, mode) = find_median_and_mode(nums);

    println!("Median: {}", median);
    println!("Mode: {}", mode);
}

fn find_median_and_mode(mut numbers: Vec<i32>) -> (f64, f64) {
    // check if empty
    if numbers.is_empty() {
        return (0.0,0.0);
    }

    // sorting the vector
    for i in 0..numbers.len() {
        for j in 0..numbers.len()-i-1 {
            if numbers[j] > numbers[j+1] {
                let temp = numbers[j];
                numbers[j] = numbers[j+1];
                numbers[j+1] = temp;
            }
        }
    }

    // calculate median
    let median = if numbers.len() % 2 == 0 {
        let mid = numbers.len() / 2;
        (numbers[mid-1] as f64 + numbers[mid] as f64) / 2.0
    } else {
        numbers[numbers.len()/2] as f64
    };

    // calculate mode on a sorted vec
    let mut mode = numbers[0];
    let mut mode_count = 1;
    let mut current_count = 1;

    for i in 1..numbers.len() {
        if numbers[i] == numbers[i-1] {
            current_count += 1;
        } else {
            current_count = 1;
        }

        if current_count > mode_count {
            mode_count = current_count;
            mode = numbers[i]
        }
    }

    (median, mode.into())
}

