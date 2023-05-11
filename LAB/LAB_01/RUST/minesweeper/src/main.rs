
pub fn main() {
    let row = 4;
    let column = 5;

    let mut board = String::new();
    board.push_str(" * *   *    *       ");
    let bytes = board.into_bytes();

    /*

    ·*·*·
    ··*··
    ··*··
    ·····

     */

    println!("\n{} {} {} {} {}", bytes[0*row+0], bytes[0*row+1], bytes[0*row+2], bytes[0*row+3], bytes[0*row+4]);
    println!("{} {} {} {} {}", bytes[1*row+1], bytes[1*row+2], bytes[1*row+3], bytes[1*row+4], bytes[1*row+5]);
    println!("{} {} {} {} {}", bytes[2*row+2], bytes[2*row+3], bytes[2*row+4], bytes[2*row+5], bytes[2*row+6]);
    println!("{} {} {} {} {}", bytes[3*row+3], bytes[3*row+4], bytes[3*row+5], bytes[3*row+6], bytes[3*row+7]);

    print!("\n{} {} {} {} {}\n", 0*row+0, 0*row+1, 0*row+2, 0*row+3, 0*row+4);
    print!("{} {} {} {} {}\n", 1*row+1, 1*row+2, 1*row+3, 1*row+4, 1*row+5);
    print!("{} {} {} {} {}\n", 2*row+2, 2*row+3, 2*row+4, 2*row+5, 2*row+6);
    println!("{} {} {} {} {}\n", 3*row+3, 3*row+4, 3*row+5, 3*row+6, 3*row+7);

    let mut board_annotated = String::new();
    let mut k:usize = 0;
    for i in 0..row {
        for j in 0..column {
            let count = count_star(i, j, &bytes, k, row, column);
            println!("colonna {}", j);
            board_annotated.push_str(count.to_string().as_str());
        }
        k += 1;
        println!();
    }

    println!();
    k = 0;
    for char in board_annotated.chars() {
        print!("{}\t", char);
        k += 1;
        if k%column == 0 {println!()}
    }

}

fn count_star(i:usize, mut j: usize, bytes: &Vec<u8>, mut k:usize, row: usize, column: usize) -> i32 {
    let mut count = 0;

    let mut column_view= 3;
    if j<=1 {column_view = 2}
    if j>=column-1 {column_view = 2}

    if j != 0 {j -= 1}

    for x in j..column_view+j {
        /*if i >= 1 {
            print!("{}({})\t", bytes[((i - 1) * row) + x + (k - 1)], ((i - 1) * row) + x + (k - 1));
            if bytes[((i - 1) * row) + x + (k - 1)] == 42 {
                count += 1;
            }
        }*/

        /*if i+1 < row {
            print!("{}({})\t", bytes[((i + 1) * row) + x + (k + 1)], ((i + 1) * row) + x + (k + 1));
            if bytes[((i + 1) * row) + x + (k + 1)] == 42 {count += 1}
        }*/
    }


    for x in i..column_view+i {
        if j > 0 {
            print!("{}({})\t", bytes[((x) * row) + j-1 + k], ((x) * row) + j-1 + k);

        }
        k += 1;
    }

    println!();

    return count;
}