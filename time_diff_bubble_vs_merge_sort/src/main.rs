// live version: https://repl.it/repls/EqualThankfulCustomer

use std::time::{SystemTime};

fn main() {
    let mut unsorted_b: Vec<i32> = Vec::new();
    for i_b in (1..10000 + 1).rev() {
        unsorted_b.push(i_b as i32);
    }

    let time_b_s = SystemTime::now();
    let _sorted_b = bubble_sort(&unsorted_b);
    let time_b_e = SystemTime::now();

    println!("BUBBLE SORT TIME: {:?}", time_b_e.duration_since(time_b_s).unwrap());
    
    let mut unsorted_m: Vec<i32> = Vec::new();
    for i_m in (1..10000 + 1).rev() {
        unsorted_m.push(i_m as i32);
    }

    let time_m_s = SystemTime::now();
    let _sorted_m = merge_sort(&unsorted_m);
    let time_m_e = SystemTime::now();

    println!("MERGE SORT TIME: {:?}", time_m_e.duration_since(time_m_s).unwrap());

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
