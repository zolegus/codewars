fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8];
    let res = two_sum(&a, 10);
    dbg!(res);
}

fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    for i in 0..numbers.len() - 1 {
        dbg!(i);
        let mut j = i + 1;
        while j < numbers.len() {
            dbg!(j);
            dbg!(numbers[i] + numbers[j]);
            if numbers[i] + numbers[j] == target {
                return (i,j)
            }
            j += 1;
        }
    }
    (0,0)
}
