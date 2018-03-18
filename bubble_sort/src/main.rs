fn main() {
    let unsorted: Vec<i32> = vec![6, 7, 1, -4, -1, 2, -2, 3, 2, 2, 0, -3, 5, 0, 4, -3, 8, 6];
    let sorted = bubble_sort(&unsorted);

    println!("BUBBLE SORT");
    println!("{:?}", unsorted);
    println!("{:?}", sorted);
}

fn bubble_sort(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();
    
    for _i in 0..vec.len() - 1 {
        for j in 0..vec.len() - 1 {
            if vec[j] > vec[j + 1] {
                let temp = vec[j + 1].clone();
                vec[j + 1] = vec[j].clone();
                vec[j] = temp.clone();
            }
        }
    }

    vec
}