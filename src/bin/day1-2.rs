use std::collections::HashMap;

fn main() {
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let input = include_str!("../../inputs/day1-1");
    let mut sum: u32 = 0;
    for line in input.lines() {
        println!("{}", line);
        let mut num_first: (isize, i32) = (1000000, 0);
        let mut num_last: (isize, i32) = (-1, 0);
        let mut figure_first: (isize, i32) = (1000000, 0);
        let mut figure_last: (isize, i32) = (-1, 0);
        for (key, val) in numbers.iter() {
            if line.contains(key) {
                let first = line.find(key).unwrap() as isize;
                if first < num_first.0 {
                    num_first.0 = first;
                    num_first.1 = *val;
                }
                let last = line.rfind(key).unwrap() as isize;
                if last > num_last.0 {
                    num_last.0 = last;
                    num_last.1 = *val;
                }
            }
            if line.contains(&val.to_string()) {
                let first = line.find(&val.to_string()).unwrap() as isize;
                if first < figure_first.0 {
                    figure_first.0 = first;
                    figure_first.1 = *val;
                }
                let last = line.rfind(&val.to_string()).unwrap() as isize;
                if last > figure_last.0 {
                    figure_last.0 = last;
                    figure_last.1 = *val;
                }
            }
        }
        println!("最初の数字:{} 最後の数字:{}", num_first.1, num_last.1);
        println!("最初の数値:{} 最後の数値:{}", figure_first.1, figure_last.1);
        let first = if num_first.0 < figure_first.0 {
            num_first.1
        } else {
            figure_first.1
        };
        let last = if num_last.0 > figure_last.0 {
            num_last.1
        } else {
            figure_last.1
        };
        if first == 0 && last == 0 {
            continue;
        }
        println!("最初:{} 最後:{}", first, last);
        let mut combined = first.to_string() + &last.to_string();
        if first == 0 {
            combined = last.to_string();
        }
        if last == 0 {
            combined = first.to_string();
        }
        let num: u32 = combined.parse().unwrap();
        println!("{:?}", num);
        sum += num;
    }
    println!("{:?}", sum);
}
