use std::{collections::HashMap, fs::File, io::Read};

use range_collections::{RangeSet, RangeSet2};

#[derive(Debug)]
struct AlmanacEntry {
    source_start: u128,
    destination_start: u128,
    range_length: u128,
}

#[derive(Debug)]
struct AlmanacMap {
    source_name: String,
    destination_name: String,
    // the key is the min and max source
    entries: HashMap<(u128, u128), AlmanacEntry>,
}

type Almanac = HashMap<String, AlmanacMap>;

fn parse_file() -> (Vec<u128>, Almanac) {
    let mut file = File::open("inputs/day_05.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let seeds: Vec<u128> = lines
        .get(0)
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect();
    //println!("Seeds are {:?}", seeds);
    let binding = contents.to_string();
    let string_maps = binding
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<String>>();

    let mut almanac: HashMap<String, AlmanacMap> = HashMap::new();
    let mut string_maps_iter = string_maps.iter();
    // ignore the first element beacuse it is the seed list
    string_maps_iter.next();
    for map_to_parse in string_maps_iter {
        let line_split = map_to_parse.split("\n").collect::<Vec<&str>>();
        let header = line_split
            .first()
            .unwrap()
            .split("-to-")
            .collect::<Vec<&str>>();
        let source = *header.get(0).unwrap();
        let destination = *header
            .get(1)
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap();
        //println!("Found a map with source: {source} and destination: {destination}");

        let mut map: AlmanacMap = AlmanacMap {
            source_name: String::from(source),
            destination_name: String::from(destination),
            entries: HashMap::new(),
        };

        for i in 1..line_split.len() {
            let mut row = line_split.get(i).unwrap().split_ascii_whitespace();
            let destination_start = row.next().unwrap().parse::<u128>().unwrap();
            let source_start = row.next().unwrap().parse::<u128>().unwrap();
            let range_length = row.next().unwrap().parse::<u128>().unwrap();
            let entry = AlmanacEntry {
                source_start,
                destination_start,
                range_length,
            };
            //println!("Found a entry {:?}", entry);
            map.entries
                .insert((source_start, source_start + range_length - 1), entry);
        }
        //println!("Built a map {:#?}", map);
        almanac.insert(String::from(source), map);
    }
    (seeds, almanac)
}

fn find_destination_code(source_code: &u128, map: &AlmanacMap) -> u128 {
    for source_interval in map.entries.keys() {
        if (source_code >= &source_interval.0) & (source_code <= &source_interval.1) {
            let entry = map.entries.get(source_interval).unwrap();
            let diff = source_code - source_interval.0;
            return entry.destination_start + diff;
        }
    }
    *source_code
}

fn find_location(seed: &u128, almanac: &Almanac) -> u128 {
    let mut map = almanac.get(&String::from("seed")).unwrap();
    let mut destination_code = find_destination_code(seed, map);
    let mut destination = &map.destination_name;
    while !destination.eq(&String::from("location")) {
        map = almanac.get(destination).unwrap();
        destination_code = find_destination_code(&destination_code, map);
        destination = &map.destination_name;
    }
    destination_code
}

pub fn solve() {
    let (seeds, almanac) = parse_file();
    let mut locations: Vec<u128> = Vec::new();
    for seed in seeds.iter() {
        locations.push(find_location(&seed, &almanac));
    }

    //println!("Seeds are {:?}", seeds);
    //println!("The Almanac is {:#?}", almanac);
    println!("Lowest location is {:?}", locations.iter().min().unwrap());
}

fn find_location_range(seed_start: &u128, range_length: &u128, almanac: &Almanac) -> u128 {
    let mut map = almanac.get(&String::from("seed")).unwrap();
    let mut destination_codes = find_destination_code_range(seed_start, range_length, map);
    let mut destination = &map.destination_name;
    while !destination.eq(&String::from("location")) {
        map = almanac.get(destination).unwrap();
        let mut new_destination_codes: Vec<(u128, u128)> = Vec::new();
        for destination_code in destination_codes {
            new_destination_codes.extend(
                find_destination_code_range(&destination_code.0, &destination_code.1, map)
            );
        }
        destination_codes = new_destination_codes;
        destination = &map.destination_name;
    }
    let mut min_locations: Vec<u128> = Vec::new();
    for destination_code in destination_codes {
        min_locations.push((destination_code.0..=(destination_code.0 + destination_code.1))
        .min()
        .unwrap());
    }
    *min_locations.iter().min().unwrap()
}

fn find_destination_code_range(
    source_code: &u128,
    range_length: &u128,
    map: &AlmanacMap,
) -> Vec<(u128, u128)> {
    //
    let mut matched_source_codes: Vec<(u128, u128)> = Vec::new();
    let mut destination_ranges: Vec<(u128, u128)> = Vec::new();

    for source_interval in map.entries.keys() {
        let source_range: RangeSet2<u128> =
            RangeSet2::from(*source_code..(source_code + range_length - 1));

        let entry = map.entries.get(source_interval).unwrap();
        let source_entry_range: RangeSet2<u128> = RangeSet2::from(source_interval.0..source_interval.1);
        // intersection between the source range for which we need the destinations and the sources in the almanac entry
        let intersection: RangeSet2<u128> = source_range.intersection(&source_entry_range);
        let inverse_intersection: RangeSet2<u128> = source_entry_range.intersection(&source_range);
        // (StartA <= EndB) and (EndA >= StartB)
        if (source_code <= &source_interval.1) & (source_code + range_length - 1 >= source_interval.0) {
        //if !((source_code > &source_interval.1) | (source_interval.0 > source_code + range_length - 1)) {
            let matching_source = *source_code.max(&source_interval.0);
            let matching_length = (source_code + range_length - 1).min(source_interval.1) - matching_source + 1;
            if matching_length == 0{
                panic!()
            }
            matched_source_codes.push((matching_source, matching_length));
            let delta_from_start = matching_source - source_interval.0;
            destination_ranges.push((entry.destination_start + delta_from_start, matching_length));
        }
        /*
        1 trovato il match con le sorgendi dell'almanacco vado a recuperare le rispettive destinazioni
        2 trovo le sorgenti che non sono presenti nell'almanacco e considero anche loro, questa volta il valore
            della destinazione è uguale a quello della entry mancante
        */
        /*if !intersection.is_empty() {
            // we have an intersection, therefore, we take the destinations of those sources
            let matching_source = intersection.boundaries()[0];
            let matching_length = intersection.boundaries()[1] - matching_source + 1;
            matched_source_codes.push((matching_source, matching_length));
            let delta_from_start = matching_source - source_interval.0;
            destination_ranges.push((entry.destination_start + delta_from_start, matching_length));
        }*/
        
    }
    // now we fill the non matched codes
    matched_source_codes.sort_by(|a, b| a.0.cmp(&b.0));
    if matched_source_codes.len() > 0 {
        if matched_source_codes.first().unwrap().0 > *source_code {
            // we have some codes that are not matched
            let matching_source = source_code;
            let matching_length = matched_source_codes.first().unwrap().0 - source_code;
            if matching_length == 0{
                panic!()
            }
            destination_ranges.push((*matching_source, matching_length));
        }
        if matched_source_codes.last().unwrap().0 + matched_source_codes.last().unwrap().1 < *source_code + range_length {
            // we have some codes that are not matched
            let matching_source = matched_source_codes.last().unwrap().0 + matched_source_codes.last().unwrap().1;
            let matching_length = *source_code + range_length - matching_source;
            if matching_length == 0{
                panic!()
            }
            destination_ranges.push((matching_source, matching_length));
        }
        // now we check for holes in the matched sources
        for i in 0..matched_source_codes.len() - 1 {
            let elem = matched_source_codes.get(i).unwrap();
            let next_elem = matched_source_codes.get(i + 1).unwrap();
            if elem.0 + elem.1  != next_elem.0 {
                let matching_source = elem.0 + elem.1;
                let matching_length = next_elem.0 - matching_source;
                if matching_length == 0{
                    panic!()
                }
                destination_ranges.push((matching_source, matching_length));
            }
        }
        destination_ranges
    } else {
        vec![(*source_code, *range_length)]
    }

}

pub fn solve_pt2() {
    let (seeds, almanac) = parse_file();
    let mut locations: Vec<u128> = Vec::new();
    for seed_range in seeds.chunks(2) {
        locations.push(find_location_range(
            &seed_range[0],
            &seed_range[1],
            &almanac,
        ));
    }
    println!("Lowest location is {:?}", locations.iter().min().unwrap());
}
