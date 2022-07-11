use std::panic;

fn main() {
    let nums = [10,20,30,40];
    let total = sum(&nums);
    println!("{:?}", total);
}
//求和计算
fn sum(arrays: &[u32]) -> Option<u32> {
    let result = panic::catch_unwind(|| {
        let mut total = 0;
        for a in arrays.iter() {
            total += a;
        }
        total
    });
    match result {
        Ok(total) => Some(total),
        Err(_) => None,
    }
}