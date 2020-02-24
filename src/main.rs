fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let line = &input();
    let mut seg: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
}

// fn sort(line: &mut Vec<i32>) {
//     line.sort_by(|a, b| a.cmp(b));
//     println!("{}", line.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
//     println!(
//         "{}",
//         line.iter()
//             .map(|&x| x.to_string() + &' '.to_string())
//             .collect::<String>()
//             .trim()
//             .to_string()
//     );
// }

// fn range(line: Vec<i32>) {
//     let mut first_num = 0;
//     if line.iter().max() == Some(&0) {
//         println!("No");
//         return;
//     }
//     for s in line {
//         if first_num != 0 && first_num >= s {
//             println!("No");
//             return;
//         }
//         first_num = s;
//     }
//     println!("Yes");
// }

// fn small_eq_large(line: Vec<i32>) {
//     match line[0] {
//         d if d > line[1] => println!("a > b"),
//         d if d == line[1] => println!("a == b"),
//         d if d < line[1] => println!("a < b"),
//         _ => (),
//     }
// }

// fn time(line: Vec<i32>) {
//     let h: i32 = line[0] / 3600;
//     let m: i32 = line[0] % 3600 / 60;
//     let s: i32 = line[0] % 60;
//     println!("{}:{}:{}", h, m, s);
// }

// fn rectangle(line: Vec<i32>) {
//     let i = line[0];
//     let j = line[1];
//     println!("{} {}", i * j, i * 2 + j * 2);
// }

// fn three(line: &str) {
//     let number: f64 = line.parse().unwrap();
//     println!("{}", number.powf(3.0));
// }

// hello world
// fn hello() {
//     println!("Hello World");
// }
