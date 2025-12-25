fn largest_joltage(bank : &str) -> u32 {
    let size = bank.len();
    if size < 2 {
        return 0;
    }

    let str_array : Vec<char> = bank.chars().collect();

    let mut index = 1;
    let mut highest : char = str_array[0];
    let mut second_highest : char = str_array[1];
    while index < size - 1 {
        if str_array[index] > highest {
            highest = str_array[index];
            second_highest = str_array[index + 1];
        } else if str_array[index] > second_highest {
            second_highest = str_array[index];
        }

        index += 1;
    }

    if str_array[index] > second_highest {
        second_highest = str_array[index];
    }

    highest.to_digit(10).unwrap() * 10 + second_highest.to_digit(10).unwrap()
}

fn atou36(array : &Vec<char>, size : usize) -> u64 {
    let mut ans : u64 = 0;
    for i in 0..size {
        ans = ans * 10 + u64::from(array[i].to_digit(10).unwrap());
    }
    return ans;
}

fn any_largest_joltage(bank : &str, size : usize) -> u64 {
    if bank.len() < size {
        return 0;
    }

    let str_array : Vec<char> = bank.chars().collect();
    let mut ans_array : Vec<char> = vec!['0';size];

    let mut index_str : usize = bank.len();
    let mut index_ans : usize = size;

    while index_ans != 0 {
        index_str -= 1;
        index_ans -= 1;

        ans_array[index_ans] = str_array[index_str];
    }

    while index_str != 0 {
        index_str -= 1;

        if str_array[index_str] >= ans_array[0] {
            let mut temp = ans_array[0];
            ans_array[0] = str_array[index_str];

            index_ans = 1;
            while index_ans < size && temp >= ans_array[index_ans] {
                let temp2 = temp;
                temp = ans_array[index_ans];
                ans_array[index_ans] = temp2;

                index_ans += 1;
            }
        }
    }

    atou36(&ans_array, size)
}

#[allow(dead_code)]
fn past_any_largest_joltage(bank : &str, size : usize) -> u64 {
    if bank.len() < size {
        return 0;
    }

    let str_array : Vec<char> = bank.chars().collect();
    let mut ans_array : Vec<char> = vec!['0';size];

    let mut index = 0;
    while index < bank.len() - size {
        for i in 0..size {
            if str_array[index] > ans_array[i] {
                for j in i..size {
                    ans_array[j] = str_array[index + j - i];
                }
                break;
            }
        }

        index += 1;
    }

    let mut i = 0;
    'outer : while index < bank.len() {
        for j in i..size {
            if str_array[index] > ans_array[j] {
                for k in j..size {
                    ans_array[k] = str_array[index + k - j];
                }

                if j + bank.len() - index <= size {
                    break 'outer;
                }

                break
            }
        }

        index += 1;
        i += 1;
    }

    atou36(&ans_array, size)
}

fn main() {
    let filename = "input";

    let mut total_output : u64 = 0;
    let mut any_total_output : u64 = 0;
    if let Ok(lines) = my_io::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            total_output += u64::from(largest_joltage(&line));
            any_total_output += any_largest_joltage(&line, 12);
        }
    }

    println!("Total output joltage with 2 batteries is = {}", total_output);
    println!("Total output joltage with 12 batteries is = {}", any_total_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_joltage_1() {
        assert_eq!(largest_joltage("987654321111111"), 98);
    }

    #[test]
    fn test_largest_joltage_2() {
        assert_eq!(largest_joltage("811111111111119"), 89);
    }

    #[test]
    fn test_largest_joltage_3() {
        assert_eq!(largest_joltage("234234234234278"), 78);
    }

    #[test]
    fn test_largest_joltage_4() {
        assert_eq!(largest_joltage("818181911112111"), 92);
    }

    #[test]
    fn test_any_largest_joltage_1() {
        assert_eq!(any_largest_joltage("987654321111111", 12), 987654321111);
    }

    #[test]
    fn test_any_largest_joltage_2() {
        assert_eq!(any_largest_joltage("811111111111119", 12), 811111111119);
    }

    #[test]
    fn test_any_largest_joltage_3() {
        assert_eq!(any_largest_joltage("234234234234278", 12), 434234234278);
    }

    #[test]
    fn test_any_largest_joltage_4() {
        assert_eq!(any_largest_joltage("818181911112111", 12), 888911112111);
    }
}
