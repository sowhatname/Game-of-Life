use std::io;
use std::io::BufRead;
use std::process::Command;

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for element in row {
            print!("{element} ");
        }
        println!();
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/c", "cls"]).status();
    } else {
        let _ = Command::new("sh").args(&["-c", "clear"]).status();
    }
}

fn find_neighbors(matrix: &Vec<Vec<i32>>, row: usize, col: usize) -> Vec<i32> {
    let mut neighbors = Vec::new();
    let rows = matrix.len();
    let cols = matrix[0].len();

    let start_row = if row > 0 {
        row - 1
    } else {
        row
    };
    let start_col = if col > 0 {
        col - 1
    } else {
        col
    };
    for i in start_row..=(row + 1).min(rows - 1) {
        for j in start_col..=(col + 1).min(cols - 1) {
            if i != row || j != col {
                neighbors.push(matrix[i][j]);
            }
        }
    }

    neighbors
}

fn count_living_person(neighbors: &Vec<i32>) -> usize {
    neighbors.iter()
        .filter(|&status| *status == 1)
        .count()
}

fn update(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_matrix = matrix.clone();
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let neighbors = find_neighbors(&matrix, i, j);
            let living_person = count_living_person(&neighbors);
            if matrix[i][j] == 1 {
                if living_person < 2 || living_person > 3{
                    new_matrix[i][j] = 0;  //人口稀少或者人口拥挤 死了
                }
            } else {
                if living_person == 3 {
                    new_matrix[i][j] = 1;
                }
            }

        }
    }
    new_matrix
}

fn menu() {
    println!("欢迎来到生命游戏！");
    println!("请输入生命游戏的初始二维网格，每行的单元格用空格分隔，行与行之间用换行分隔，以0代表死亡，1代表存活");
    let matrix = vec![
        vec![1, 0, 1],
        vec![0, 1, 0],
        vec![1, 0, 1],
    ];
    println!("例如（对于3×3网格)：");
    print_matrix(&matrix);
}

fn main() {
    menu();
    let mut matrix = Vec::new();
    println!("请逐行输入矩阵函数，每行元素用空格分隔，输入空行结束：");
    for line in io::stdin().lock().lines() {
        let line = line.expect("读取行失败");
        if line.is_empty() {
            break;
        }
        let row: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if !row.is_empty() {
            matrix.push(row);
        }
    }

    let mut input = String::new();
    println!("请输入游戏的演化步数（例如：10）：");
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");
    let steps: i32 = input
        .trim()
        .parse()
        .expect("输入无效");
    let mut input = String::from("\n");
    let mut i = 1;
    while input.contains("\n") && i <= steps {
        clear_screen();
        matrix = update(matrix);
        println!("当前步数：{i}");
        print_matrix(&matrix);
        println!("按回车键进入下一步...");

        io::stdin()
            .read_line(&mut input)
            .expect("读取输入失败");
        i += 1;
    }
}
