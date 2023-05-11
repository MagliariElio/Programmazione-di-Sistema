use std::thread;
use std::time::Instant;
use itertools::Itertools;

const LENGTH_DATA_ARRAY:usize = 5;

fn calculate(data_array: Vec<&i32>, operations: Vec<&&str>) -> i32 {
    if data_array.len() != LENGTH_DATA_ARRAY {
        panic!("The array has not the right length as specified");
    }

    let mut i = 0;                                  // index for operations array
    let mut result = *data_array[0] as f64;
    let data = &data_array[1..data_array.len()];
    for value in data {
        match *operations[i] {
            "+" => result += f64::from(**value),
            "-" => result -= f64::from(**value),
            "x" => result *= f64::from(**value),
            "/" => result /= f64::from(**value),
            _ => panic!("This operation doesn't exist"),
        }
        i += 1;
        if i > operations.len() {
            panic!("Error, the index of operations array is greater of its len");
        }
    }

    return result as i32;
}

fn compute_string_operation(data: &Vec<&i32>, operations: Vec<&&str>) -> String {
    let mut result = String::new();
    let mut i = 0;              // index of operations array
    for value in data {
        result.push_str(&format!("{}", value));
        if i < operations.len() {
            result.push_str(&format!(" {} ", operations[i]));
            i += 1;
        }
    }

    return result;
}

fn calculate_all_permutation(perm_data: &[Vec<&i32>], perm_operations: &Vec<Vec<&&str>>, thread_number: &str) {
    let mut sequence: Vec<String> = Vec::new();
    for data in perm_data {
        for operation in perm_operations.clone() {
            if calculate(data.clone(), operation.clone()) == 10 {
                sequence.push(compute_string_operation(data, operation.clone()));
            }
        }
    }

    println!("sequence of thread {}: {:?}\nSequence length: {}\n", thread_number, sequence, sequence.len());
}

fn main() {
    let start_time = Instant::now();

    let data:[i32; LENGTH_DATA_ARRAY] = [2, 7, 2, 2, 1];
    let operations = ["+", "-", "x", "/"];

    let perm_data: Vec<Vec<&i32>> = data.iter().permutations(data.len()).collect();
    let perm_operations: Vec<Vec<&&str>> = operations.iter().combinations_with_replacement(operations.len()).collect();

    //calculate_all_permutation(&perm_data, &perm_operations, "0");
    //calculate_all_permutation(&[vec![&2, &1, &7, &2, &2]], &vec![vec![&"+", &"x", &"x", &"/"]], "0");

    thread::scope(|s| {
        s.spawn(|| {calculate_all_permutation(&perm_data[0..20], &perm_operations, "1")});
        s.spawn(|| {calculate_all_permutation(&perm_data[20..40], &perm_operations, "2")});
        s.spawn(|| {calculate_all_permutation(&perm_data[40..60], &perm_operations, "3")});
        s.spawn(|| {calculate_all_permutation(&perm_data[60..80], &perm_operations, "4")});
        s.spawn(|| {calculate_all_permutation(&perm_data[80..100], &perm_operations, "5")});
        s.spawn(|| {calculate_all_permutation(&perm_data[100..120], &perm_operations, "6")});
    });

    let end_time = Instant::now();
    let execution_time = end_time.duration_since(start_time);
    println!("Program Execution Time: {:?}", execution_time);
}




/*
+   -   x   /

2   +   7   +   2   +   2   +   1
2   +   7   +   2   +   2   -   1
2   +   7   +   2   +   2   x   1
2   +   7   +   2   +   2   /   1

2   +   7   +   2   -   2   +   1
2   +   7   +   2   -   2   -   1
2   +   7   +   2   -   2   x   1
2   +   7   +   2   -   2   /   1

2   +   7   +   2   x   2   +   1
2   +   7   +   2   x   2   -   1
2   +   7   +   2   x   2   x   1
2   +   7   +   2   x   2   /   1

2   +   7   +   2   /   2   +   1
2   +   7   +   2   /   2   -   1
2   +   7   +   2   /   2   x   1
2   +   7   +   2   /   2   /   1

1.  arriva fino al penultimo valore dell'array
2.  applica l'operazione passata con l'ultimo elemento
3.  ritorna il valore

            +
            -
            x
            /

    2   2   2   2   2   2   2   2   2   2   2   2   2   2   2   2
    +   +   +   +   +   +   +   +   +   +   +   +   +   +   +   +
    7   7   7   7   7   7   7   7   7   7   7   7   7   7   7   7
    +   +   +   +   +   +   +   +   -   -   -   -   -   -   -   -
    2   2   2   2
    +   -   x   /
    2   2   2   2
    +   -   x   /
    1   1   1   1

*/