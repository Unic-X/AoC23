use std::{fs, collections::{HashSet, HashMap}};


/* 
*  Day 4
*  Both the parts need rewrite 
*/

fn part_1(inp:String)->u32{

    // Create Vector of strings
    let lines:Vec<&str> = inp.lines().collect::<Vec<&str>>();
    lines.iter().map(|line| {
        let cards = line.split(":").nth(1).unwrap();
        let real = cards.split("|").collect::<Vec<&str>>();
        let winning: HashSet <u32> =  HashSet::from_iter(real.iter().nth(0).unwrap().split_whitespace().map(|num_s| num_s.parse::<u32>().unwrap()));
        
        let ours : HashSet<u32> =   HashSet::from_iter(real.iter().nth(1).unwrap().split_whitespace().map(|num_s| num_s.parse::<u32>().unwrap()));
        let inter = winning.intersection(&ours).count();
        println!("{inter:?}");
        if inter>0 {
           (2 as u32).pow((winning.intersection(&ours).count() as u32).saturating_sub(1))
        }else {
            0
        }
        

    }).sum()

}


fn part_2(inp:String)->u32{

    // Create Vector of strings
    let lines:Vec<&str> = inp.lines().collect::<Vec<&str>>();
    let mut map:HashMap<u32,u32> = HashMap::new();
    lines.iter().map(|line| {

        let cards = line.split(":").nth(1).unwrap();
        let num = line.split(":").nth(0).unwrap().split("Card").nth(1).unwrap().trim().parse::<u32>().unwrap();

        let real = cards.split("|").collect::<Vec<&str>>();
        let winning: HashSet <u32> =  HashSet::from_iter(real.iter().nth(0).unwrap().split_whitespace().map(|num_s| num_s.parse::<u32>().unwrap())); 
        let ours : HashSet<u32> =   HashSet::from_iter(real.iter().nth(1).unwrap().split_whitespace().map(|num_s| num_s.parse::<u32>().unwrap()));
        let inter = winning.intersection(&ours).count();
        if inter>0 {
                for i in (num+1)..  (num + 1 + inter as u32 ) {
                  if i<=218 {
                    map.entry(i).and_modify(|v| *v+=1 ).or_insert(1);
                }
                }
            match map.clone().get(&num) {
                Some(val) => {
                for i in (num+1)..  (num + 1 + inter as u32 ) {
                    if i<=218 {
                         map.entry(i).and_modify(|v| *v+=val).or_insert(1);
                    }
                }
                    (inter as u32).saturating_sub(1) * val
                },
                None => {
                    map.entry(num).or_insert(0);
                    (inter as u32).saturating_sub(1)
                },
            }
        }else {
            if map.get(&num) == None {
                  map.entry(num).or_insert(0);
            }
            0
        }

    }).sum::<u32>();

    map.iter().map(|(num,&val)| {
    
       println!("{:?} {:?}                   {:?}",num,val+1,map.keys().max()); 
        val+1
        
    }).sum()

}


fn main() {
    let examp = fs::read_to_string("input/day_4");
    match examp {
        Ok(ok) => {
            println!("{:?}",part_2(ok));
        }
        Err(e) => {
            println!("{}",e);
        }
        
    }
}

