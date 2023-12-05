use std::{fs::File, io::Read, collections::HashMap};

#[derive(Debug)]
struct AlmanacEntry {
    source_start: u32,
    destination_start: u32,
    range_length: u32
}

#[derive(Debug)]
struct AlmanacMap {
    source_name: String,
    destination_name: String,
    // the key is the min and max source
    entries: HashMap<(u32, u32), AlmanacEntry>
}

type Almanac = HashMap<String, AlmanacMap>;

fn parse_file() -> (Vec<u32>, Almanac) {
    let mut file = File::open("inputs/day_05_example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let seeds: Vec<u32> = lines
        .get(0)
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    //println!("Seeds are {:?}", seeds);
    let binding = contents.to_string();
    let string_maps = binding.split("\n\n").map(String::from).collect::<Vec<String>>();
    
    
    let mut almanac: HashMap<String, AlmanacMap> = HashMap::new();
    let mut string_maps_iter = string_maps.iter();
    // ignore the first element beacuse it is the seed list
    string_maps_iter.next();
    for map_to_parse in string_maps_iter {
        let line_split = map_to_parse.split("\n").collect::<Vec<&str>>();
        let header = line_split.first().unwrap().split("-to-").collect::<Vec<&str>>();
        let source = *header.get(0).unwrap();
        let destination = *header.get(1).unwrap().split(" ").collect::<Vec<&str>>().get(0).unwrap();
        //println!("Found a map with source: {source} and destination: {destination}");

        let mut map: AlmanacMap = AlmanacMap { 
            source_name: String::from(source), 
            destination_name: String::from(destination), 
            entries: HashMap::new() 
        };

        for i in 1..line_split.len() {
            let mut row = line_split.get(i).unwrap().split_ascii_whitespace();
            let destination_start = row.next().unwrap().parse::<u32>().unwrap();
            let source_start = row.next().unwrap().parse::<u32>().unwrap();
            let range_length = row.next().unwrap().parse::<u32>().unwrap();
            let entry = AlmanacEntry {
                source_start,
                destination_start,
                range_length
            };
            //println!("Found a entry {:?}", entry);
            map.entries.insert((source_start, source_start + range_length - 1), entry);
        }
        //println!("Built a map {:#?}", map);
        almanac.insert(String::from(source), map);
    }
    (seeds, almanac)
}

fn find_destination_code(source_code: &u32, map: &AlmanacMap) -> u32 {
    0
}

fn find_location(seed: &u32, almanac: &Almanac) -> u32 {
    let mut map = almanac.get(&String::from("seed")).unwrap();
    let mut destination_code = find_destination_code(seed, map);
    let mut destination = &map.destination_name;
    while destination.eq(&String::from("location")) {
        map = almanac.get(destination).unwrap();
        destination_code = find_destination_code(&destination_code, map);
        destination = &map.destination_name;
    }
    destination_code
}

pub fn solve() {
    let (seeds, almanac) = parse_file();
    let mut locations: Vec<u32> = Vec::new();
    for seed in seeds.iter() {
        locations.push(find_location(&seed, &almanac));
    }

    //println!("Seeds are {:?}", seeds);
    //println!("The Almanac is {:#?}", almanac);
    println!("Lowest location is {:?}", locations.iter().min().unwrap());
}
