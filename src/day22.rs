use std::{fs, time, collections::{HashMap, VecDeque}};

use nom::{IResult, 
    multi::separated_list0, bytes::complete::tag, 
    combinator::{map_res, recognize},
    sequence::{tuple, separated_pair}, character::complete::digit1,
};

/*
* Day 22
* Will do other days later
* Have no clue brain rot has begun 
* ACcidently deleted once
*/

#[derive(Debug,PartialEq,Hash,Eq)]
struct Coordinates{
    x:usize,
    y:usize,
    z:usize
}

#[derive(Debug,PartialEq,Hash,Eq)]
struct Brick{
    start:Coordinates,
    end:Coordinates
}


impl Brick {

    fn fall(&self,amount:usize)->Brick{

        let new_start = Coordinates{
            x:self.start.x,
            y:self.start.y,
            z:self.start.z.saturating_sub(amount),
        };

        let new_end = Coordinates{
            x:self.end.x,
            y:self.end.y,
            z:self.end.z.saturating_sub(amount),

        };

        Brick {
            start:new_start,
            end:new_end
        }
        //Amount that will fall will always be in Z direction 
    }

}

trait Bricks{
    fn find_intersecting(&self,brick:&Brick)->Vec<&Brick>;
    fn intersecting(&self,brick:&Brick)->Vec<&Brick>;
    fn above(&self,brick:&Brick)->Vec<&Brick>;
    fn below(&self,brick:&Brick)->Vec<&Brick>;
    fn no_other_support(&self,brick_b:&Brick,brick_a:&Brick,fell:&HashMap<&Brick,bool>)->bool;

    //Part 2
    fn jenga(&self)->Vec<usize>;
}

impl Bricks for Vec<Brick>{
    //Detect if  X,Y of any brick lies between the X,Y of other brick 

    fn intersecting(&self,brick:&Brick)->Vec<&Brick>{
        self.into_iter()
            .filter(|&b| {
                b.start.x <= brick.end.x &&
                b.end.x >= brick.start.x &&
                b.start.y <= brick.end.y &&
                b.end.y >= brick.start.y
            }).collect::<Vec<_>>()
    }

    fn find_intersecting(&self,brick:&Brick)->Vec<&Brick>{
        self.intersecting(brick).into_iter()
            .filter(|&b| //Remove other bricks that are above the current brick
                b.end.z < brick.start.z
            ).collect()
    }


    //Vec of every Brick above the desired brick
    fn above(&self,brick:&Brick)->Vec<&Brick> {
         self.intersecting(brick).into_iter()
            .filter(|&b| //Remove other bricks that are above the current brick
                b.start.z == brick.end.z + 1
            ).collect()

    }

    //Vec of every Brick below the desired brick
    fn below(&self,brick:&Brick)->Vec<&Brick> {
         self.intersecting(brick).into_iter()
            .filter(|&b| //Remove other bricks that are above the current brick
                b.end.z == brick.start.z - 1
            ).collect()
    }
    fn no_other_support(&self,brick_b:&Brick,brick_a:&Brick,fell:&HashMap<&Brick,bool>)->bool{
        match fell.get(brick_b) {
            Some(value) => {
                if *value==true {
                    self.below(&brick_a)
                        .iter()
                        .filter(|&b| *b!=brick_b)
                        .filter(|&b| !*fell.get(b).unwrap_or(&false)) 
                        .collect::<Vec<_>>()
                        .is_empty()
                }else {
                    return false;
                }
            }
            None=>{
                return false;
            }

        }

           
    }

    //Part 2
    
    fn jenga(&self)->Vec<usize> {
        let mut all_falls = Vec::new();

        for start in self.into_iter(){

            let mut q = VecDeque::new();


            q.push_back(start);
            let mut fell = HashMap::new();

            while let Some(curr) = q.pop_front() {
                let above = self.above(curr);

                fell.insert(curr, true); 
                if above.is_empty(){
                    break;
                }
                
                for above_brick in above.into_iter(){
                    if self.no_other_support(curr,above_brick,&fell){
                        q.push_back(above_brick);        
                         //If no other support the brick will fall
                    }
                    fell.insert(above_brick, false);
                }
            }
          
        fell.insert(start, false); //Set the starting index to be false
         all_falls.push(
                fell.iter().filter(|&(_,v)| *v==true).collect::<Vec<_>>().len()
            );

        }

        all_falls
    }

}


//----------------Parsing stuff---------------------
fn parse_u64(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), |s: &str| s.parse::<usize>())(input)
}

fn coordinate_parser(input: &str) -> IResult<&str, Coordinates> {
    let (input, (x, _, y, _, z)) = tuple((
        parse_u64,
        tag(","),
        parse_u64,
        tag(","),
        parse_u64,
    ))(input)?;

    Ok((input, Coordinates { x, y, z }))
}

fn parse_brick(input:&str)->IResult<&str,Brick>{
    let (input,(start,end)) = separated_pair(coordinate_parser, tag("~"), coordinate_parser)(input)?;

    Ok((input,Brick{start,end}))
}

fn parse(input:&str)->IResult<&str,Vec<Brick>>{
    let (input,bricks) = separated_list0(tag("\n"), parse_brick)(input)?;
    Ok((input,bricks))
}
//--------------------------------------------------



fn part_1(inp:String)->(usize,Vec<Brick>){
    let (_,mut bricks) = parse(&inp).unwrap();
    
    let now = time::Instant::now();
    //Arrange blocks starting from bottom 
    bricks.sort_by_key(|b| b.start.z.min(b.end.z));

    //One can see that the start is always the smaller one that is in every thing x,y,z in most cases in example
    
    for i in 0..bricks.len() {
        let brick = &bricks[i];

        // Get intersecting bricks based on the x, y coordinates of end and start
        let intersecting_bricks = bricks.find_intersecting(&brick);

        // Get the highest of them
        let highest = intersecting_bricks.iter().max_by_key(|&b| b.end.z);

        // Check if the brick has any lower brick that is intersecting
        // if none don't change any z value
        if let Some(high_bric) = highest {
            let amount_to_fall = brick.start.z.saturating_sub(high_bric.end.z + 1);

            // Update the brick in the vector
            bricks[i] = brick.fall(amount_to_fall);
        }else{
            //If there is no other brick below the desired brick
            //Simply move the brick to the bottom 
            let delta = brick.end.z - brick.start.z;
            bricks[i].start.z = 1;
            bricks[i].end.z = bricks[i].start.z + delta;
        }
    }



    let removable = bricks.iter().filter(|&b| {
        let above = bricks.above(&b);

        above.iter().any(|&b2| // If the brick above has a brick which can be supported by other brick 
            bricks.below(&b2)
            .iter()
            .filter(|&b3| *b3!=b)
            .collect::<Vec<_>>()
            .is_empty()
        )
        }
    ).collect::<Vec<_>>();
    
    println!("Done in ({:?})",now.elapsed());
    (removable.len(),bricks)

}

fn part_2(input:String)->usize{
    let (_,settled_bricks) = part_1(input);

    let all_falls = settled_bricks.jenga();

    let now = time::Instant::now();

    dbg!(&all_falls);

    println!("Done in ({:?})",now.elapsed());
    println!("Length: {:?}",all_falls.len());
    all_falls.iter().sum()
}   


fn main() {
    let examp = fs::read_to_string("input/day_22");
    match examp {
        Ok(ok) => {
            let p2 = part_2(ok);
            println!("Part 2 Solution : {p2:?}");
        
        }
        Err(e) => {
            println!("{}",e);
        }
        
    }
}
