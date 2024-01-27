use std::{fs,ops::Add, collections::HashMap};
use nom_locate::LocatedSpan;
use nom::{multi::{separated_list1, many1}, 
    character::complete::{char,newline}, 
    IResult,
    branch::alt,
    combinator::map, sequence::tuple,
};


/*
* Day 23
* Will do other days later
* Have no clue brain rot has begun
*/


type Span<'a> = LocatedSpan<&'a str>;


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
impl Point{
    fn new(x:usize,y:usize)->Self{
        Point { x, y }
    }
}


fn with_xy(span: &Span) -> Point {
    let x = span.get_column();
    let y = span.location_line() as usize;
    Point::new(x, y)
}



//----------------Parsing stuff---------------------

fn parse_forest(input: Span) -> IResult<Span, LegendMap> {
    map(char('#'), |_| LegendMap{
        legend:Legend::Forest,
        point:with_xy(&input)
    })(input)

}

fn parse_path(input: Span) -> IResult<Span, LegendMap> {
    

    map(char('.'), |i| LegendMap{
        legend:Legend::Path,
        point:with_xy(&input),
    })(input)

}

fn parse_slope(input: Span) -> IResult<Span, LegendMap> {
    let parse_direction = alt((
        map(char('^'), |_| Slope::North),
        map(char('>'), |_| Slope::East),
        map(char('v'), |_| Slope::South),
        map(char('<'), |_| Slope::West),
    ));
    
    map(parse_direction,|slope| LegendMap{
        point:with_xy(&input),
        legend:Legend::Slope(slope)}
    )(input)

}

fn parse_map(input: Span) -> IResult<Span, Vec<LegendMap>> {
    let (input, map) = many1(alt((
        |i| parse_forest(i),
        |i| parse_path(i),
        |i| parse_slope(i),
    )))(input)?;
    Ok((input, map))
}


fn parse(input: Span) -> IResult<Span, Vec<Vec<LegendMap>>> {
    let (input, maps) = separated_list1(newline, |i| parse_map(i))(input)?;
    todo!();
}

//--------------------------------------------------

fn part_1(inp:String)->usize{
    let span = Span::new(&inp);

    parse(span).unwrap();
    2
}

fn part_2(input:String)->usize{
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
