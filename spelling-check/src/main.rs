fn prefix_i(input1: &str, input2: &str)  -> usize {
    let mut i = 0;
    while input1.chars().nth(i) == input2.chars().nth(i) {
        i += 1;
    }
    i + 1
}

fn suffix_i(input1: &str, input2: &str) -> usize {
    let mut i = 0;
    while input1.chars().rev().nth(i) == input2.chars().rev().nth(i) {
        i += 1;
    }
    input1.len() - i
}

fn main() {
    let input1 = "abcdxxxef";
    let input2 = "abcdxxef";

    let prefix = prefix_i(input1, input2);
    let suffix = suffix_i(input1, input2);
    let mut indices = vec![];

    for i in suffix..prefix + 1 {
        indices.push(i);
    } 
    println!("{}", indices.len());
    println!("{:?}", indices);
}

