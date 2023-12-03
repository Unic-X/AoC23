use std::fs;


/* 
*  Day 1
*  Shittiest way to figure out the solution but works
*/

fn part_1(inp:String){

    // Create Vector of strings
    let lines:Vec<&str> = inp.lines().collect::<Vec<&str>>();
    
    let sum = lines.iter().
        fold(0, |acc,&x| acc+ {
            let mut numero :u32 = 0;
            //Match Forward String
            for char in x.chars(){
                match char {
                    c if c.is_numeric() => {
                        numero += 10*c.to_digit(10).unwrap();
                        break;
                    }
                    _ => {} //Do Nothing
                }
            }

            for char in x.chars().rev(){
                match char {
                    c if c.is_numeric() => {
                        numero+= c.to_digit(10).unwrap();
                        break;
                    }
                    _ => {} //Do Nothing
                }
            }

        numero
    });


    println!("{:?}",sum);
}



fn part_2(inp:&str){
 let lines:Vec<&str> = inp.lines().collect::<Vec<&str>>();

    let mut lines_:Vec<String> = vec![];
    for line in lines {
        let mut line2 = line.to_string();

        //Shittiest way to figure out the values inside a string 

        line2 = line2
            .replace("nine", "n9e")
            .replace("eight", "e8t")
            .replace("seven", "s7n")
            .replace("six", "s6x")
            .replace("five", "f5e")  
            .replace("four", "f4r")  
            .replace("three", "t3e")
            .replace("two", "t2o")
            .replace("one", "o1e");


        lines_.push(line2);
    }

    part_1(lines_.iter().map(
            |x|
            {
                let mut new_line = x.to_string();
                new_line.push_str("\n");
                new_line
            }
        ).collect()
    )

}

fn main() {
    let examp = fs::read_to_string("input/day_1");
    match examp {
        Ok(ok) => {
            part_2(&ok);
        }
        Err(e) => {
            println!("{}",e);
        }
        
    }
}

