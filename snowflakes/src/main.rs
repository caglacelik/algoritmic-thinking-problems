use std::io;
use::std::collections::{LinkedList, HashMap};

const SIZE: usize = 100_000;

pub fn identify_identical(snowflakes: &HashMap<usize, LinkedList<[i32;6]>>) -> bool {
    for val in snowflakes.values() {
        for (i, snowflake1) in val.iter().enumerate() {
            for snowflake2 in val.iter().skip(i + 1) {
                if are_identical(snowflake1, snowflake2) {
                    return true;
                }
            }
        }
    }
    return false
}

pub fn identical_right(snowflake1: &[i32;6], snowflake2: &[i32;6], start: usize) -> bool {
    let mut iter = snowflake2.iter().cycle().skip(start);

    for i1 in snowflake1 {
        if let Some(i2) = iter.next() {
            if i1 != i2 {
                return false;
            }
        }
    }
    return true
}

pub fn identical_left(snowflake1: &[i32;6], snowflake2: &[i32;6], start: usize) -> bool {
    let mut iter = snowflake2.iter().rev().cycle().skip(5 - start);
    
    for i1 in snowflake1 {
        if let Some(i2) = iter.next() {
            if i1 != i2 {
                return false;
            }
        }
    }

    return true;
}

pub fn are_identical(snowflake1: &[i32;6], snowflake2: &[i32;6]) -> bool {
    for i in 0..6 {
        if identical_right(snowflake1, snowflake2, i){
            return true;
        }

        if identical_left(snowflake1, snowflake2, i) {
            return true;
        }
    }
    return false;
}

pub fn code(snowflake: &[i32;6]) -> usize {
    (snowflake.iter().sum::<i32>() as usize) % SIZE
}

#[cfg(test)]
mod test {
    use crate::identical_left;

    use super::*;

    #[test]
    fn test_identical_right() {
        let snowflake1 = [10, 20, 30, 40, 50, 60];
        let snowflake2 = [40, 50, 60, 10, 20, 30];

        assert!(identical_right(&snowflake1, &snowflake2, 3));
    }

    #[test]
    fn test_identical_left() {
        let snowflake1 = [300, 5, 70, 6, 1, 2];
        let snowflake2 = [2, 1, 6, 70, 5, 300];

        assert!(identical_left(&snowflake1, &snowflake2, 5));
    }

    #[test]
    fn test_are_identical() {
        let snowflake1 = [8, 22, 3, 40, 67, 9];
        let snowflake2 = [40, 67, 9, 8, 22, 3];

        assert!(are_identical(&snowflake1, &snowflake2));
    }

    #[test]
    fn test_code() {
        let snowflake = [40099, 59, 6, 75, 33, 5];
        assert_eq!(code(&snowflake), 40277);
    }
}

fn main() {
        println!("Enter the number of snowflakes you will write:");
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        let int_num:i32 = num.trim().parse().unwrap();

        let mut snowflakes: HashMap<usize, LinkedList<[i32;6]>> = HashMap::new();
        println!("Enter the snowflakes:");
        for _i in 0..int_num {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let mut snowflake = [0;6];
            for (index, flake) in input.split_whitespace().enumerate() {
                snowflake[index] = flake.trim().parse().unwrap();
            }

            let code = code(&snowflake);
            match snowflakes.get_mut(&code) {

                Some(list) => {
                    list.push_back(snowflake)
                },
                None => {
                    let mut snowflake_list = LinkedList::new();
                    snowflake_list.push_front(snowflake);
                    snowflakes.insert(code, snowflake_list);
                }
            }
        }

        if identify_identical(&snowflakes) {
            println!("Twin snowflakes found.");
        }
        else {
            println!("No two snowflakes are alike.");
        }
        println!("{:?}", snowflakes);

}


