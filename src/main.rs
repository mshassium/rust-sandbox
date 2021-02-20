use std::io;
use std::io::Read;

fn main() {
    println!("Hello world!!!")
}




// Codeforces block start
fn a344()-> io::Result<()> {
    let mut stdin = io::stdin();
    let mut count_str: String = String::new();
    stdin.read_line(&mut count_str);
    let count: i32 = count_str.trim().parse::<i32>().unwrap();
    let mut result: i32 = 0;
    let mut prev_val: String = String::new();
    for i in 0..count  {
        let mut next_val = String::new();
        stdin.read_line(&mut next_val);
        if next_val.trim() != prev_val.trim(){
            result+=1;
        }
        prev_val = next_val;
    }
    println!("{}", result);
    Ok(())
}
fn a677() -> io::Result<()> {
    let stdin = io::stdin();
    let mut first_line: String = String::new();
    let mut second_line: String = String::new();
    stdin.read_line(&mut first_line);
    let v: Vec<&str> = first_line.trim().split(" ").collect();
    let n: i32 = v[0].to_string().parse::<i32>().unwrap();
    let h: i32 = v[1].to_string().parse::<i32>().unwrap();
    stdin.read_line(&mut second_line);
    let heights: Vec<&str> = second_line.trim().split(" ").collect();
    let mut count: i32 = n;
    for next_h in heights {
        if next_h.to_string().parse::<i32>().unwrap() > h {
            count+=1;
        }
    }
    println!("{}", count);
    Ok(())
}
fn a734() -> std::io::Result<()> {
    let mut stdin = std::io::stdin();
    let mut count: String = String::new();
    let mut results: String = String::new();
    stdin.read_line(&mut count)?;
    stdin.read_line(&mut results)?;
    let v: Vec<&str> = results.split("").collect();
    let mut count_a = 0;
    let mut count_b = 0;
    for next in v.iter() {
        if next.clone() == "A" {
            count_a += 1;
        } else if next.clone() == "D" {
            count_b += 1;
        }
    }
    if count_a > count_b {
        println!("Anton")
    } else if count_a < count_b {
        println!("Danik")
    } else {
        println!("Friendship");
    }
    Ok(())
}
// Codeforces block end

