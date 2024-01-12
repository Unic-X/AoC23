use std::{fs,ops::Add, collections::HashMap};

use nom::{multi::{separated_list1, many1}, 
    character::complete::{char,newline}, 
    IResult,
    branch::alt,
    combinator::map,
};

/*
* Day 23
* Will do other days later
* Have no clue brain rot has begun
*/



#[derive(Debug,Clone,Copy)]
struct Point{
    x:usize,
    y:usize,
}

#[derive(Debug)]
enum Slope{
    North,
    West,
    South,
    East
}

#[derive(Debug)]
enum Legend {
    Path,
    Forest,
    Slope(Slope),
}

#[derive(Debug)]
struct LegendMap{
    legend:Legend,
    point:Point
}


#[derive(Debug)]
struct Map{
    all_points:Vec<HashMap<Point,Legend>>
}

impl Add for Point{
    type Output = Point;
    fn add(self,other:Point) -> Point{
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


//----------------Parsing stuff---------------------

fn parse_forest<'a,'b>(input: &'a str,point:&'b mut Point) -> IResult<&'a str, LegendMap> {
    map(char('#'), |_| LegendMap{
        legend:Legend::Forest,
        
        point:*point,

    })(input)
}

fn parse_path<'a,'b>(input: &'a str,point:&'b mut Point) -> IResult<&'a str, LegendMap> {
    map(char('.'), |_| LegendMap{
        legend:Legend::Path,
        point:*point,
    })(input)

}

fn parse_slope<'a,'b>(input: &'a str,point:&'b mut Point) -> IResult<&'a str, LegendMap> {
    let parse_direction = alt((
        map(char('^'), |_| Slope::North),
        map(char('>'), |_| Slope::East),
        map(char('v'), |_| Slope::South),
        map(char('<'), |_| Slope::West),
    ));
    map(parse_direction,|slope| LegendMap{
        point:*point,
        legend:Legend::Slope(slope)}
    )(input)

}

fn parse_map<'a>(input: &'a str, point: &mut Point) -> IResult<&'a str, Vec<LegendMap>> {
    let (input, map) = many1(alt((
        {parse_forest(input, point)},
        parse_path(input, point),
        parse_slope(input, point),
    )))(input)?;


    Ok((input, map))
}


fn parse(input: &str) -> IResult<&str, Vec<Vec<LegendMap>>> {
    let mut initial_point = Point { x: 0, y: 0 };
    let (_, maps) = separated_list1(newline, |i| parse_map(i, &mut initial_point))(input)?;
    dbg!(maps);
    todo!();
}

//--------------------------------------------------

fn part_1(inp:String)->usize{
    let (_,all_mapping) = parse(&inp).unwrap();
    dbg!(all_mapping);
    2
}

fn part_2(_input:String)->usize{
 todo!()   
}   


fn main() {
    let examp = fs::read_to_string("input/day_23");
    match examp {
        Ok(ok) => {
            let p1 = part_1(ok);
            println!("Part 1 Solution : {p1:?}");
        
        }
        Err(e) => {
            println!("{}",e);
        }
        
    }
}
