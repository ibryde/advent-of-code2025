use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[allow(dead_code)]
fn get_pwr10(id : i64) -> (i64, i64) {
    let mut nb_digits : i64 = 1;
    let mut pwr : i64 = 1;
    let mut number : i64 = id;

    while number >= 10 {
        nb_digits += 1;

        if nb_digits % 2 == 0 {
            pwr *= 10;
        }

        number /= 10;
    }

    (nb_digits, pwr)
}

#[allow(dead_code)]
fn is_bad_id_even(id : i64, pwr10 : i64) -> bool{
    let half1 = id / pwr10;
    let half2 = id % pwr10;

    // println!("Test with {}, pwr10 = {}, half1 = {}, half2 = {}"
    //     , id, pwr10, half1, half2);

    half1 == half2
}

/*
 * Bad ID only appears if a patterns repeat twice
 */
#[allow(dead_code)]
fn bad_id_even(id : i64) -> bool {
    let (digits_nb, pwr10) = get_pwr10(id);

    // println!("Test with {}, digits_nb = {}", id, digits_nb);

    digits_nb % 2 != 1 && is_bad_id_even(id, pwr10)
}

#[allow(dead_code)]
fn count_bad_ids_even(mut begin : i64, end : i64) -> i64 {
    let mut count : i64 = 0;

    while begin <= end {
        let (digits_nb, pwr10) = get_pwr10(begin);
        if digits_nb % 2 == 1 {
            begin = pwr10 * pwr10 * 10;
        } else {
            // println!("pwr10 = {}", pwr10);
            let mut threeshold = end;
            if pwr10 < 10000000000 && pwr10 * pwr10 - 1 < end {
                threeshold = pwr10 * pwr10 - 1; // avoid overflow
            }

            while begin <= threeshold {
                if is_bad_id_even(begin, pwr10) {
                    count += begin;
                }

                begin += 1;
            }
        }
    }

    count
}

fn is_a_pattern(s : &str, pattern_size : usize) -> bool {
    let str_array : Vec<char> = s.chars().collect();

    let size = str_array.len();
    let mut index = pattern_size;
    while index < size {
        let mut pattern_index = 0;
        while pattern_index < pattern_size {
            if str_array[index] != str_array[pattern_index] {
                return false;
            }
            pattern_index += 1;
            index += 1;
        }
    }

    true
}

fn is_bad_id(id : &str) -> bool {
    let size = id.len();
    for i in 1..(size/2 + 1) {
        if size % i != 0 {
            continue; 
        }

        if is_a_pattern(id, i) {
            return true;
        }
    }

    false
}

fn count_bad_ids(mut begin : i64, end : i64) -> i64 {
    let mut count : i64 = 0;

    while begin <= end {
        if is_bad_id(&begin.to_string()) {
            // println!("is bad id = {}", begin);
            count += begin;
        }

        begin += 1;
    }

    count
}

//      30000000000 to 
// from 29997908954 to 39997908954
fn main() {
    let mut incorrects_ids = 0;

    let path = Path::new("input");
    let mut file = File::open(&path).unwrap();

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut range = buf.chars().position(|c| c == ',');
    while !range.is_none() {
        let current = buf[..range.unwrap()].to_string();
        let separator = current.chars().position(|c| c == '-').unwrap();

        println!("current = {}", current);
        let begin = current[..separator].parse::<i64>().unwrap();
        let end = current[(separator+1)..].parse::<i64>().unwrap();

        incorrects_ids += count_bad_ids(begin, end);

        buf = buf[(range.unwrap() + 1)..].to_string();
        range = buf.chars().position(|c| c == ',');
    }

    println!("The count of bad ids is : {}", incorrects_ids + count_bad_ids(262248430,262271846));
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isodd1_even() {
        assert_eq!(bad_id_even(101), false);
    }

    #[test]
    fn test_not_bad_id_even1_even() {
        assert_eq!(bad_id_even(1011), false);
    }

    #[test]
    fn test_not_bad_id_even2_even() {
        assert_eq!(bad_id_even(111011), false);
    }

    #[test]
    fn test_bad_id_1_even() {
        assert_eq!(bad_id_even(1010), true);
    }

    #[test]
    fn test_bad_id_2_even() {
        assert_eq!(bad_id_even(10101010), true);
    }

    #[test]
    fn test_bad_id_3_even() {
        assert_eq!(bad_id_even(99), true);
    }

    #[test]
    fn test_not_bad_range_odd_1_even() {
        assert_eq!(count_bad_ids_even(100, 999), 0);
    }

    #[test]
    fn test_not_bad_range_1_even() {
        assert_eq!(count_bad_ids_even(12, 20), 0);
    }

    #[test]
    fn test_bad_range_1_even() {
        assert_eq!(count_bad_ids_even(99, 105), 99);
    }

    #[test]
    fn test_bad_range_2_even() {
        assert_eq!(count_bad_ids_even(11, 22), 11 + 22);
    }

    #[test]
    fn test_bad_range_3_even() {
        assert_eq!(count_bad_ids_even(95, 115), 99);
    }

    #[test]
    fn test_bad_range_4_even() {
        assert_eq!(count_bad_ids_even(998, 1012), 1010);
    }

    #[test]
    fn test_bad_range_5_even() {
        assert_eq!(count_bad_ids_even(1188511880,1188511890), 1188511885);
    }

    #[test]
    fn test_bad_id_1() {
        assert_eq!(is_bad_id("1010"), true);
    }

    #[test]
    fn test_bad_id_2() {
        assert_eq!(is_bad_id("101010"), true);
    }

    #[test]
    fn test_bad_id_3() {
        assert_eq!(is_bad_id("1111111"), true);
    }

    #[test]
    fn test_bad_id_4() {
        assert_eq!(is_bad_id("12341234"), true);
    }

    #[test]
    fn test_bad_range_1() {
        assert_eq!(count_bad_ids(11, 22), 11 + 22);
    }

    #[test]
    fn test_bad_range_2() {
        assert_eq!(count_bad_ids(95, 115), 99 + 111);
    }

    #[test]
    fn test_bad_range_3() {
        assert_eq!(count_bad_ids(998, 1012), 999 + 1010);
    }

    #[test]
    fn test_bad_range_4() {
        assert_eq!(count_bad_ids(1188511880, 1188511890), 1188511885);
    }

    #[test]
    fn test_bad_range_10() {
        assert_eq!(count_bad_ids(222220, 222224), 222222);
    }

    #[test]
    fn test_bad_range_5() {
        assert_eq!(count_bad_ids(446443, 446449), 446446);
    }

    #[test]
    fn test_bad_range_6() {
        assert_eq!(count_bad_ids(38593856, 38593862), 38593859);
    }

    #[test]
    fn test_bad_range_7() {
        assert_eq!(count_bad_ids(565653, 565659), 565656);
    }

    #[test]
    fn test_bad_range_8() {
        assert_eq!(count_bad_ids(824824821, 824824827), 824824824);
    }

    #[test]
    fn test_bad_range_9() {
        assert_eq!(count_bad_ids(2121212118, 2121212124), 2121212121);
    }

    #[test]
    fn test_not_bad_id_1() {
        assert_eq!(is_bad_id("12345234"), false);
    }
}
