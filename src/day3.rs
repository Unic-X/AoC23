use std::{collections::HashMap, fs};

// Day 3
// Trying to do these problems using functional programming but i suck hard
//

fn part_1(inp: String) -> u32{
    // Create Vector of strings
    let lines: Vec<&str> = inp.lines().collect::<Vec<&str>>();
    let lines_char: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let sum: u32 = lines
        .iter()
        .enumerate()
        .map(|(r, line)| {
            let mut num_s = String::new();
            let mut adjacent = false;
            line.chars()
                .enumerate()
                .filter_map(|(c, char)| {
                    if let Some(valid) = char.to_digit(10) {
                        for neighbour in [
                            (r + 1, c),
                            (r + 1, c + 1),
                            (r, c + 1),
                            (r.saturating_sub(1), c),
                            (r, c.saturating_sub(1)),
                            (r.saturating_sub(1), c.saturating_sub(1)),
                            (r.saturating_sub(1), c + 1),
                            (r + 1, c.saturating_sub(1)),
                        ] {
                            let (n_row, n_col) = (neighbour.0, neighbour.1);

                            if n_row < line.len() && n_col < lines.len() {
                                let neighbour_char = lines_char[n_row][n_col];
                                match neighbour_char {
                                    '&' | '%' | '*' | '#' | '@' | '/' | '=' | '$' | '+' | '-' => {
                                        adjacent = true;
                                    }
                                    _ => {}
                                }
                            }
                        }

                        num_s.push_str(&valid.to_string());
                        if let Some(next_char) = lines_char.iter().nth(r).unwrap().iter().nth(c + 1)
                        {
                            if !(next_char.is_digit(10)) && adjacent {
                                let num2 = num_s.clone();
                                Some(num2.parse::<u32>().unwrap_or(0))
                            } else {
                                None
                            }
                        } else if c + 1 == line.len() && adjacent {
                            Some(num_s.parse::<u32>().unwrap_or(0))
                        } else {
                            None
                        }
                    } else {
                       adjacent = false;
                        num_s.clear();
                        None
                    }
                })
                .sum::<u32>() // Sum of numbers that matches with the condition
        })
        .sum();
    return sum;
}





fn part_2(inp: String) -> u32{
    // Create Vector of strings
    let lines: Vec<&str> = inp.lines().collect::<Vec<&str>>();
    let lines_char: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    lines
        .iter()
        .enumerate()
        .map(|(r, line)| {
            let mut num_s = String::new();
            let mut adjacent = false;
            let mut near_star: (usize, usize) = (usize::MAX, usize::MAX);
            line.chars()
                .enumerate()
                .filter_map(|(c, char)| {
                    if let Some(valid) = char.to_digit(10) {
                        for neighbour in [
                            (r + 1, c),
                            (r + 1, c + 1),
                            (r, c + 1),
                            (r.saturating_sub(1), c),
                            (r, c.saturating_sub(1)),
                            (r.saturating_sub(1), c.saturating_sub(1)),
                            (r.saturating_sub(1), c + 1),
                            (r + 1, c.saturating_sub(1)),
                        ] {
                            let (n_row, n_col) = (neighbour.0, neighbour.1);

                            if n_row < line.len() && n_col < lines.len() {
                                let neighbour_char = lines_char[n_row][n_col];
                                match neighbour_char {
                                    '*' => {
                                        adjacent = true;
                                        near_star = (n_row, n_col);
                                    }
                                    _ => {}
                                }
                            }
                        }

                        num_s.push_str(&valid.to_string());
                        let next_char = lines_char.iter().nth(r).unwrap().iter().nth(c + 1);
                        if next_char != None {
                            if !(next_char.unwrap().is_digit(10)) && adjacent {
                                map.entry(near_star)
                                    .or_insert_with(Vec::new)
                                    .push(num_s.parse::<u32>().unwrap_or(1));

                                Some(num_s.parse::<u32>().unwrap_or(1))
                            } else {
                                None
                            }
                        } else if c + 1 == line.len() && adjacent {
                            map.entry(near_star)
                                .or_insert_with(Vec::new)
                                .push(num_s.parse::<u32>().unwrap_or(1));
                            Some(num_s.parse::<u32>().unwrap_or(1))
                        } else {
                            None
                        }
                    } else {
                        adjacent = false;
                        num_s.clear();
                        None
                    }
                })
                .sum::<u32>() // Sum of numbers that matches with the condition
        })
        .sum::<u32>();

    let sum2: u32 = map
        .iter()
        .filter(|(_, val_v)| val_v.len() > 1)
        .map(|(_, val_v)| val_v.iter().fold(1, |acc, &x| acc * x))
        .sum();

    return sum2;
}

fn main() {
    let examp = fs::read_to_string("input/day_3");
    match examp {
        Ok(ok) => {
            part_1(ok);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}


#[test]
fn test_1() {
    let input = String::from("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
    assert_eq!(4361,part_1(input));

}


#[test]
fn test_2() {
    let input = String::from("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
    assert_eq!(467835,part_2(input));
}

