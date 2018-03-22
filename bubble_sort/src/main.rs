// Bubble Sort Algorithm
// live version: https://repl.it/repls/MuffledSnowLaboratory

fn main() {
    // let unsorted: Vec<f64> = vec![6.7456, 7.1456, 1.010, -4.54, -1.95467, 2.104, -2.2324, 3.0154, 2.105, 2.015, 0.156, -3.9273, 5.21, 0.001, 4.014, -3.436, 8.92893, 6.7394];
    // let unsorted: Vec<f32> = vec![6.7, 7.1, 1.0, -4.5, -1.9, 2.1, -2.2, 3.0, 2.3, 2.0, 0.0, -3.9, 5.2, 0.0, 4.0, -3.4, 8.9, 6.7];
    // let unsorted: Vec<u32> = vec![6, 7, 1, 4, 1, 2, 2, 3, 2, 2, 0, 3, 5, 0, 4, 3, 8, 6];
    // let unsorted: Vec<usize> = vec![6, 7, 1, 4, 1, 2, 2, 3, 2, 2, 0, 3, 5, 0, 4, 3, 8, 6];
    // let unsorted: Vec<i32> = vec![6, 7, 1, -4, -1, 2, -2, 3, 2, 2, 0, -3, 5, 0, 4, -3, 8, 6];
    let unsorted: Vec<isize> = vec![6, 7, 1, -4, -1, 2, -2, 3, 2, 2, 0, -3, 5, 0, 4, -3, 8, 6];
    let sorted = bubble_sort(&unsorted);

    println!("BUBBLE SORT");
    println!("{:?}", unsorted);
    println!("{:?}", sorted);
}

fn bubble_sort<T>(vec: &Vec<T>) -> Vec<T>
    where T: 
        std::ops::Mul<Output = T> +
        std::cmp::PartialOrd +
        Copy {
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
