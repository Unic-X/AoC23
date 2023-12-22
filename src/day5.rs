use std::{fs, ops::RangeInclusive};

use nom::{IResult, 
    sequence::tuple, 
    bytes::complete::{tag, take_until}, 
    multi::{separated_list0, separated_list1}, 
    character::{complete::digit1, streaming::space1, complete}
};

/* 
*  Day 5
*  Trying to parse with nom
*  Most unoptimized code ever
*/

fn parse_map(input:&str)->IResult<&str,Mapping>{
    let (input,(dest,_,source,_,delta)) = tuple((complete::u64,space1,complete::u64,space1,complete::u64))(input)?; // Explicitly hardcoded the u64 :(
    let map = Mapping::new(source as usize,dest as usize,delta as usize);
    Ok((input,map))
}

fn mappings_parse(input:&str)->IResult<&str,Vec<Mapping>>{
    let (input,_) = tuple((take_until("\n"),tag("\n")))(input)?; //Skip over the first line 
    let (input,maps) = separated_list1(tag("\n"), parse_map)(input)?; 
    Ok((input,maps))
}

fn parse(input:&str)->IResult<&str,(Vec<usize>,Almanac)>{
    //Parse out the SeedID!!
    let (input,(_,seeds)) = tuple((tag("seeds: "),separated_list0(tag(" "), digit1)))(input)?;
    let seeds:Vec<usize> = seeds.iter()
        .map(|&seed|
            seed.parse::<usize>().unwrap()
        ).collect();
    //Skipping next two new lines
    let (input,_) = tag("\n\n")(input)?;
    //The input now contains the mappings like seed-to-soil etc
    let (input,mappings) = separated_list0(tag("\n\n"), mappings_parse)(input)?;
    //Parsing the Mappings 
    let rev_mappings: Vec<Vec<Mapping>> = mappings.into_iter()
        .map(|mut map| {
            map.reverse();
            map
        })
        .collect();

    let almanac= Almanac::new(rev_mappings);

    Ok((input,(seeds,almanac)))

}


#[derive(Debug)]
struct Mapping{
    source : RangeInclusive<usize>,
    destination : RangeInclusive<usize>,
}

impl Mapping{
    fn new(source:usize,dest:usize,delta:usize)->Self{
        let s = source;
        let d = dest;
        let del = delta-1;

        Self { source: s..=s+del , destination: d..=d+del  }
    }

    fn sc_contains(&self,val:usize)->bool{
        self.source.contains(&val)
    }

    fn map_val(&mut self,val:usize)->usize{
        if self.sc_contains(val) {
            let idx = self.source.position(|key| key==val).unwrap();
            return self.destination.clone().nth(idx).unwrap()
        }
        val
    }

}

struct Almanac{
    mappings: Vec<Vec<Mapping>>
}

impl Almanac {
    fn new(mappings:Vec<Vec<Mapping>>)->Self{
        Self { mappings }
    }
 
    fn to_location(&mut self,seed:usize)->usize{
        let mut l = seed;

        for maps in self.mappings.iter_mut() { //Iterate over the mappings like seed-to-soil etc
            for map in maps.iter_mut() {       //Iterate over every map in a mapping
                dbg!(&l,&map);
                if map.sc_contains(l) {    //Check if source range contains the source or not
                    l = map.map_val(l);    // If it does change the value to the dest range value

                    break;                  
                }
            }
        println!("Changing value to {l:?}\n");
        }
        return l;
    }   
}


fn part_1(inp:String)->usize{
    let (_,(seeds,mut all_maps)) = parse(&inp).unwrap();

    let location:Vec<usize> = seeds.iter()
        .map(|&seed| all_maps.to_location(seed)).collect();
    *location.iter().min().unwrap()

}


fn main() {
    let examp = fs::read_to_string("input/day_5");
    match examp {
        Ok(ok) => {
            println!("{:?}",part_1(ok));
        }
        Err(e) => {
            println!("{}",e);
        }
        
    }
}

