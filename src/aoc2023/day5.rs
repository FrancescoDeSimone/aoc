use fancy_regex::Regex;
use rayon::prelude::*;

#[derive(Eq, Hash,PartialEq)]
struct SoilRange{
    start: u64,
    stop: u64,
}

fn get_in_range(range:&Vec<(SoilRange,SoilRange)>, seed:u64) -> u64 {
    for (k, v) in range {
        if seed >= k.start && seed <= k.stop {
            return v.start + (seed - k.start);
        }
    }
    return seed
}

fn calculate_seeds(seeds:Vec<u64>) -> Vec<SoilRange> {
    let mut res = Vec::new();
    for i in (0..seeds.len()-1).step_by(2) {
        res.push(
            SoilRange{
                start: seeds[i],
                stop: seeds[i]+seeds[i+1]
            }
        )
    }
    res
}

fn insert_into_map(range:&mut Vec<(SoilRange,SoilRange)>, lines:Vec<&str>) {
    for line in lines.into_iter().filter(|e| !e.is_empty()) {
        let parts = line.trim().split(" ").collect::<Vec<&str>>();
        let dest = parts[0].parse::<u64>().unwrap();
        let source = parts[1].parse::<u64>().unwrap();
        let length = parts[2].parse::<u64>().unwrap();
        range.push((SoilRange{start: source, stop: source+length-1},
            SoilRange{start: dest, stop: dest+length-1}));
    }
}

pub fn part_1(input: String) -> u64 {
      let regex = Regex::new(r"(?m)^(\w+):\s*(.+)\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*|$)").unwrap();

    let mut res = u64::MAX;
    let mut seeds = Vec::new();
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();
    for cap in regex.captures_iter(&input) {
        let cap = cap.unwrap();
        for i in (1..cap.len()).step_by(2){
            match cap.get(i).unwrap().as_str(){
                "seeds" => {
                    seeds = cap.get(i+1).unwrap().as_str().split(" ")
                                .filter(|e| !e.is_empty()).map(|e| e.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                }
                "seed-to-soil" => {
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut seed_to_soil, line);
                }
                "soil-to-fertilizer" => {
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut soil_to_fertilizer, line);

                }
                "fertilizer-to-water" => {
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut fertilizer_to_water, line);
                }
                "water-to-light" => {
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut water_to_light, line);
                }
                "light-to-temperature"=>{
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut light_to_temperature, line);
                }
                "temperature-to-humidity" =>{
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut temperature_to_humidity, line);
                }
                "humidity-to-location" =>{
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut humidity_to_location, line);
                }
                _ => unimplemented!()
            }
        }
    }
    for seed in seeds {
        let soil = get_in_range(&seed_to_soil, seed);
        let fertilizer = get_in_range(&soil_to_fertilizer, soil);
        let water = get_in_range(&fertilizer_to_water, fertilizer);
        let light = get_in_range(&water_to_light, water);
        let temperature = get_in_range(&light_to_temperature, light);
        let humidity = get_in_range(&temperature_to_humidity, temperature);
        let location = get_in_range(&humidity_to_location, humidity);
        res = res.min(location);
    }
    res
}

pub fn part_2(input: String) -> u64 {
      let regex = Regex::new(r"(?m)^(\w+):\s*(.+)\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*?(?:\n{2,}|$))\n\n([^ ]+)..*\n([\s\S]*|$)").unwrap();

    let mut res = u64::MAX;
    let mut seeds = Vec::new();
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();
    for cap in regex.captures_iter(&input) {
        let cap = cap.unwrap();
        for i in (1..cap.len()).step_by(2){
            match cap.get(i).unwrap().as_str(){
                "seeds" => {
                    seeds =  calculate_seeds(cap.get(i+1).unwrap().as_str().split(" ")
                                .filter(|e| !e.is_empty()).map(|e| e.parse::<u64>().unwrap()).collect::<Vec<u64>>());
                }
                "seed-to-soil" => {
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut seed_to_soil, line);
                }
                "soil-to-fertilizer" => {
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut soil_to_fertilizer, line);

                }
                "fertilizer-to-water" => {
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut fertilizer_to_water, line);
                }
                "water-to-light" => {
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut water_to_light, line);
                }
                "light-to-temperature"=>{
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut light_to_temperature, line);
                }
                "temperature-to-humidity" =>{
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut temperature_to_humidity, line);
                }
                "humidity-to-location" =>{
                    let line = cap.get(i+1).unwrap().as_str().split("\n").collect::<Vec<&str>>();
                    insert_into_map(&mut humidity_to_location, line);
                }
                _ => unimplemented!()
            }
        }
    }
    for seed_range in seeds {
        res = res.min((seed_range.start..seed_range.stop)
            .into_par_iter().fold(|| u64::MAX,|min, seed| {
            let soil = get_in_range(&seed_to_soil, seed);
            let fertilizer = get_in_range(&soil_to_fertilizer, soil);
            let water = get_in_range(&fertilizer_to_water, fertilizer);
            let light = get_in_range(&water_to_light, water);
            let temperature = get_in_range(&light_to_temperature, light);
            let humidity = get_in_range(&temperature_to_humidity, temperature);
            let location = get_in_range(&humidity_to_location, humidity);
            min.min(location)
        }).min().unwrap());
    }
    res
}

#[test]
fn check_part1(){
    let oracle = 35;
    let input = std::fs::read_to_string("src/aoc2023/tests/day5").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2(){
    let oracle = 46;
    let input = std::fs::read_to_string("src/aoc2023/tests/day5").unwrap();
    assert_eq!(oracle, part_2(input));
}
