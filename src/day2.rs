use std::fs;

use regex::Regex;




fn part_1(inp: String) {
    let lines: Vec<&str> = inp.lines().collect();

    let pattern = r"(\d+) (green|blue|red)";
    let re = Regex::new(pattern).unwrap();
    let mut game_id = 0;
    let sum: u32 = lines.iter().fold(0, |acc, &x| {
        game_id+=1; 

        let all_events_valid = x.trim().split(":").nth(1).map_or(false, |events| {
            events.trim().split(";").all(|event| {
                let mut green_count = 0;
                let mut red_count = 0;
                let mut blue_count = 0;

                event.trim().split(",").all(|balls| {
                    if let Some(caps) = re.captures(balls) {
                        let count: u32 = caps[1].parse().unwrap();
                        let color = &caps[2];
                        match color {
                            "green" => green_count += count,
                            "red" => red_count += count,
                            "blue" => blue_count += count,
                            _ => {}
                        }
                        true
                    } else {
                        false
                    }
                });

                blue_count <= 14 && red_count <= 12 && green_count <= 13
            })
        });

        if all_events_valid {
            acc + game_id
        } else {
            acc
        }
    });

    println!("Sum: {:?}", sum);
}



// Part 2 needs huge amount of refactor 

fn part_2(inp: String) {
    let lines: Vec<&str> = inp.lines().collect();

    let pattern = r"(\d+) (green|blue|red)";
    let re = Regex::new(pattern).unwrap();
    let sum: u32 = lines.iter().fold(0, |acc, &x| {

        let mut max_red=0; let mut max_blue=0; let mut max_green = 0;

        let all_events_valid = x.trim().split(":").nth(1).map(|events| {
                events.trim().split(";").map(|event| {
                let mut green_count = 0;
                let mut red_count = 0;
                let mut blue_count = 0;

                event.trim().split(",").map(|balls| {
                    if let Some(caps) = re.captures(balls) {
                        let count: u32 = caps[1].parse().unwrap();
                        let color = &caps[2];
                        match color {
                            "green" => green_count += count,
                            "red" => red_count += count,
                            "blue" => blue_count += count,
                            _ => {}
                        }
                        true;
                    } else {
                        false;
                    }
                    if green_count>max_green {
                       
                        max_green = green_count;
                    }
                    if red_count>max_red {
                        max_red = red_count;
                    }
                    if blue_count>max_blue {
                        max_blue = blue_count;
                    }
                   
                }).count();
                (max_red,max_green,max_blue)
                
            }).count();

        
        (max_red,max_green,max_blue)

        });
        let valid = all_events_valid.unwrap() ;
        let power = valid.0 * valid.1 *valid.2;
            acc + power
    });

    println!("Sum: {:?}", sum);
}

fn main() {
    let examp = fs::read_to_string("input/day_2");
    match examp {
        Ok(ok) => {
            part_2(ok);
        }
        Err(e) => {
            println!("{}",e);
        }
        
    }
}

