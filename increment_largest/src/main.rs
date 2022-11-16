fn main() {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        for x in digits.iter_mut().rev() {
            let sum = *x + 1;
            *x = sum % 10;
            if sum < 10 {
                return digits;
            }
        }
        [&vec![1], &digits[..]].concat()
    }

    let test_vec = Vec::from([1, 10, 3, 4]);
    println!("{:?}", plus_one(test_vec))
}
