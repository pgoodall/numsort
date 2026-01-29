fn median(n: &mut Vec<i32>) {
    n.sort();
    let length = n.len();
    let half = length / 2;
    if length % 2 == 0 {
        let median_value = (&n[half] + &n[half + 1]) / 2 ;
        println!("The median value is: {}", median_value);
    } else {
        let median_value = &n[half];
        println!("The median value is: {}", median_value);
    }
}

fn mode(n: Vec<i32>) {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for num in &n {
        let count = map.entry(num).or_insert(0);
        *count +=1;
    }

    let most = map.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();

    println!("The most frequent value is: {most:?}");

}

fn main() {
    let mut nums: Vec<i32> = vec![2, 5, 19, 8, 2, 7, 13, 16, 21];
    println!("The length is: {}", nums.len());
    println!("The original list: {:?}", nums);

    median(&mut nums);
    mode(nums)
}
