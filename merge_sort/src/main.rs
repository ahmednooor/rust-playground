// Merge Sort Algorithm
// -- created using pseudocode from https://www.ee.ryerson.ca/~courses/coe428/sorting/mergesort.html
// live version: https://repl.it/repls/UnfinishedMarvelousWebportal

fn main() {
    // let unsorted: Vec<f64> = vec![6.7456, 7.1456, 1.010, -4.54, -1.95467, 2.104, -2.2324, 3.0154, 2.105, 2.015, 0.156, -3.9273, 5.21, 0.001, 4.014, -3.436, 8.92893, 6.7394];
    // let unsorted: Vec<f32> = vec![6.7, 7.1, 1.0, -4.5, -1.9, 2.1, -2.2, 3.0, 2.3, 2.0, 0.0, -3.9, 5.2, 0.0, 4.0, -3.4, 8.9, 6.7];
    // let unsorted: Vec<u32> = vec![6, 7, 1, 4, 1, 2, 2, 3, 2, 2, 0, 3, 5, 0, 4, 3, 8, 6];
    // let unsorted: Vec<usize> = vec![6, 7, 1, 4, 1, 2, 2, 3, 2, 2, 0, 3, 5, 0, 4, 3, 8, 6];
    // let unsorted: Vec<i32> = vec![6, 7, 1, -4, -1, 2, -2, 3, 2, 2, 0, -3, 5, 0, 4, -3, 8, 6];
    let unsorted: Vec<isize> = vec![6, 7, 1, -4, -1, 2, -2, 3, 2, 2, 0, -3, 5, 0, 4, -3, 8, 6];
    let sorted = merge_sort(&unsorted);

    println!("MERGE SORT");
    println!("{:?}", unsorted);
    println!("{:?}", sorted);
}

fn merge<T>(mut left_half_array: Vec<T>, mut right_half_array: Vec<T>) -> Vec<T>
    where T: 
        std::ops::Mul<Output = T> +
        std::cmp::PartialOrd +
        Copy {
    let mut result_array: Vec<T> = Vec::new();
    
    loop {
        if left_half_array.len() > 0
                && right_half_array.len() > 0
                && left_half_array[0] <= right_half_array[0] {
            result_array.push(left_half_array.remove(0));

        } else if left_half_array.len() > 0
                && right_half_array.len() > 0
                && left_half_array[0] > right_half_array[0] {
            result_array.push(right_half_array.remove(0));

        } else if left_half_array.len() < 1
                && right_half_array.len() > 0 {
            result_array.push(right_half_array.remove(0));

        } else if right_half_array.len() < 1
                && left_half_array.len() > 0 {
            result_array.push(left_half_array.remove(0));

        } else {
            break;
            
        }
    }

    result_array
}

fn merge_sort<T>(vec: &Vec<T>) -> Vec<T> 
    where T: 
        std::ops::Mul<Output = T> +
        std::cmp::PartialOrd +
        Copy {
    let vec = vec.clone();
    let arr_len = vec.len() - 1 as usize;
    
    if arr_len == 0 {
        vec
        
    } else {
        let middle: usize = arr_len / 2 as usize;
        
        let mut left_half_array: Vec<T> = Vec::new();
        for i in 0..(middle + 1) {
            left_half_array.push(vec[i]);
        }
        left_half_array = merge_sort(&left_half_array);
        
        let mut right_half_array: Vec<T> = Vec::new();
        for i in (middle + 1)..(arr_len + 1) {
            right_half_array.push(vec[i]);
        }
        right_half_array = merge_sort(&right_half_array);
        
        let result_array = merge(left_half_array, right_half_array);
        result_array

    }
}
