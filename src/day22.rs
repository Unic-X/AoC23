use std::fs;

use nom::{IResult, 
    multi::separated_list0, bytes::complete::tag, 
    combinator::{map_res, recognize},
    sequence::{tuple, separated_pair}, character::complete::digit1,
};

/*
* Day 22
* Will do other days later
* Have no clue brain rot has begun 
*/

#[derive(Debug,PartialEq)]
struct Coordinates{
    x:usize,
    y:usize,
    z:usize
}

#[derive(Debug,PartialEq)]
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
}

impl Bricks for Vec<Brick>{
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
                b!=brick && b.end.z < brick.start.z
            ).collect()
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



fn part_1(inp:String){
    let (_,mut bricks) = parse(&inp).unwrap();
    

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
            let delta = brick.end.z - brick.start.z;

            bricks[i].start.z = 1;
            bricks[i].end.z = bricks[i].start.z + delta;
        }
    }




}


fn main() {
    let examp = fs::read_to_string("input/day_22");
    match examp {
        Ok(ok) => {
            part_1(ok);
        }
        Err(e) => {
            println!("{}",e);
        }
        
    }
}
