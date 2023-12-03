use std::{fs, collections::HashMap};


// Day 3
// Trying to do these problems using functional programming but i suck hard
//

fn part_1(inp:String){

    // Create Vector of strings
    let lines:Vec<&str> = inp.lines().collect::<Vec<&str>>();
    let lines_char :Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let sum :u32 = lines
        .iter()
        .enumerate()
        .map(|(r,line)| {
            let mut num_s = String::new();
            let mut adjacent = false;
            line.chars()
                .enumerate()
                .filter_map(|(c,char)| {
                    if let Some(valid) = char.to_digit(10){
                        let r_2 = r as i32;
                        let c_2 = c as i32;
                        for neighbour in [(r_2+1,c_2),(r_2+1,c_2+1),(r_2,c_2+1),(r_2-1,c_2),(r_2,c_2-1),(r_2-1,c_2-1),(r_2-1,c_2+1),(r_2+1,c_2-1)] {

                            let (n_row,n_col) = (neighbour.0 as usize, neighbour.1 as usize);

                            if n_row < line.len() && n_col < lines.len() {
                                
                                let neighbour_char = lines_char[n_row][n_col];
                                match neighbour_char {
                                    '&' | '%' | '*' | '#' | '@' | '/' | '=' | '$' | '+' | '-' =>{
                                    adjacent = true;
                                }
                                _ => {}
                                }

                            }
                                                   
                        }

                    num_s.push_str(&valid.to_string());
                    let next_char = lines_char.iter().nth(r).unwrap().iter().nth(c+1);
                    if next_char!= None{
                            if !(next_char.unwrap().is_digit(10)) {
                                if adjacent {
                                    let num2 =  num_s.clone();
                                    print!("{num_s}  ");
                                    Some(num2.parse::<u32>().unwrap_or(0))
                                }
                                else{
                                    None
                                }
                        }else {
                            None
                        }
                    }else if c+1 == line.len() && adjacent {
                            print!("{num_s}  ");
                            Some(num_s.parse::<u32>().unwrap_or(0))
                        }
                        else {
                        None
                    }
                    }else {
                        adjacent = false;
                        num_s.clear();
                        None
                    }
            })
            .sum::<u32>()  // Sum of numbers that matches with the condition 

        })
        .sum();
    print!("{sum:?}");

}


fn part_2(inp:String){

    // Create Vector of strings
    let lines:Vec<&str> = inp.lines().collect::<Vec<&str>>();
    let lines_char :Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut map:HashMap<(usize,usize), Vec<u32>> = HashMap::new();

    let sum :u32 = lines
        .iter()
        .enumerate()
        .map(|(r,line)| {
            let mut num_s = String::new();
            let mut adjacent = false;
            line.chars()
                .enumerate()
                .filter_map(|(c,char)| {
                    if let Some(valid) = char.to_digit(10){
                        let r_2 = r as i32;
                        let c_2 = c as i32;
                        for neighbour in [(r_2+1,c_2),(r_2+1,c_2+1),(r_2,c_2+1),(r_2-1,c_2),(r_2,c_2-1),(r_2-1,c_2-1),(r_2-1,c_2+1),(r_2+1,c_2-1)] {

                            let (n_row,n_col) = (neighbour.0 as usize, neighbour.1 as usize);

                            if n_row >= 0 && n_row < line.len() && n_col >= 0 && n_col < lines.len() {
                                
                                let neighbour_char = lines_char[n_row][n_col];
                                match neighbour_char {
                                    '*' => {
                                        if map.contains_key(&(n_row,n_col)) {
                                        }
                                        adjacent = true;
                                    }
                                _ => {}
                                }

                            }
                                                   
                        }

                    num_s.push_str(&valid.to_string());
                    let next_char = lines_char.iter().nth(r).unwrap().iter().nth(c+1);
                    if next_char!= None{
                            if !(next_char.unwrap().is_digit(10)) {
                                if adjacent {
                                    let num2 =  num_s.clone();
                                    print!("{num_s}  ");
                                    Some(num2.parse::<u32>().unwrap_or(0))
                                }
                                else{
                                    None
                                }
                        }else {
                            None
                        }
                    }else if c+1 == line.len() && adjacent {
                            Some(num_s.parse::<u32>().unwrap_or(0))
                        }
                        else {
                        None
                    }
                    }else {
                        adjacent = false;
                        num_s.clear();
                        None
                    }
            })
            .sum::<u32>()  // Sum of numbers that matches with the condition 

        })
        .sum();
    print!("{sum:?}");

}

fn main() {
    let examp = fs::read_to_string("input/day_3");
    match examp {
        Ok(ok) => {
            part_2(ok);
        }
        Err(e) => {
            println!("{}",e);
        }
        
    }
}

