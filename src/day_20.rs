use std::{fs::File, io::Read, collections::{HashMap, VecDeque}, fmt::Debug, vec};

#[derive(Debug)]
enum ComponentType {
    Broadcast,
    Conjunction,
    FlipFlop,
    Output
}

trait Component: Debug {
    fn component_type(&self) -> ComponentType;
    fn signal(&mut self, input: &(String, bool)) -> Option<Vec<(String, bool)>>;
    fn destinations(&self) -> &Vec<String>;
}

#[derive(Debug)]
struct FlipFlop {
    state: bool,
    destinations: Vec<String>
}

impl FlipFlop {
    fn from_str(input: &str) -> (String, FlipFlop) {
        let split = input.strip_prefix("%").unwrap().split(" -> ").collect::<Vec<&str>>();
        let name = String::from(*split.get(0).unwrap()).trim().to_string();
        let destinations = split.get(1).unwrap().split(", ").into_iter().map(String::from).collect::<Vec<String>>();
        (name, FlipFlop { state: false, destinations })
    }
}

impl Component for FlipFlop {
    fn signal(&mut self, input: &(String, bool)) -> Option<Vec<(String, bool)>> {
        let pulse = input.1;
        let output = if !pulse {
            self.state = !self.state;
            Some(self.destinations.iter().map(|d| (d.clone(), self.state)).collect::<Vec<(String, bool)>>())
        } else {
            None
        };
        output
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::FlipFlop
    }

    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }
}

#[derive(Debug)]
struct Conjunction {
    recent_pulse: HashMap<String, bool>,
    destinations: Vec<String>
}

impl Conjunction {
    fn from_str(input: &str) -> (String, Conjunction) {
        let split = input.strip_prefix("&").unwrap().split(" -> ").collect::<Vec<&str>>();
        let name = String::from(*split.get(0).unwrap()).trim().to_string();
        let destinations = split.get(1).unwrap().split(", ").into_iter().map(String::from).collect::<Vec<String>>();
        // from the string specification we don't know the input modules of this Conjunction node
        // therefore, we put it as empty
        (name, Conjunction { recent_pulse: HashMap::new(), destinations })
    }
}

impl Component for Conjunction {
    fn signal(&mut self, input: &(String, bool)) -> Option<Vec<(String, bool)>> {
        let (name, pulse) = input;
        *self.recent_pulse.entry(name.to_string()).or_insert(false) = *pulse;
        let output_pulse = !self.recent_pulse.values().all(|x| *x);
        Some(self.destinations.iter().map(|d| (d.clone(), output_pulse)).collect::<Vec<(String, bool)>>())
    }


    fn component_type(&self) -> ComponentType {
        ComponentType::Conjunction
    }


    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }
}

#[derive(Debug)]
struct Broadcast {
    destinations: Vec<String>
}

impl Component for Broadcast {
    fn signal(&mut self, input: &(String, bool)) -> Option<Vec<(String, bool)>> {
        let pulse = input.1;
        Some(self.destinations.iter().map(|d| (d.clone(), pulse)).collect::<Vec<(String, bool)>>())
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Broadcast
    }


    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }
}

impl Broadcast {
    fn from_str(input: &str) -> (String, Broadcast) {
        let split = input.split(" -> ").collect::<Vec<&str>>();
        let name = String::from(*split.get(0).unwrap()).trim().to_string();
        let destinations = split.get(1).unwrap().split(", ").into_iter().map(String::from).collect::<Vec<String>>();
        (name, Broadcast { destinations })
    }
}

#[derive(Debug)]
struct Output {
    destinations: Vec<String>
}

impl Output {
    fn new() -> Output {
        Output { destinations: Vec::new() }
    }
}

impl Component for Output {
    fn component_type(&self) -> ComponentType {
        ComponentType::Output
    }

    fn signal(&mut self, _input: &(String, bool)) -> Option<Vec<(String, bool)>> {
        None
    }

    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }
}

fn parse_content(content: &str) -> HashMap<String, Box<dyn Component>> {
    let mut components: HashMap<String, Box<dyn Component>> = HashMap::new();
    let mut conjunctions: HashMap<String, Conjunction> = HashMap::new();
    for line in content.lines() {
        if line.starts_with("broadcaster") {
            let (name, component) = Broadcast::from_str(line);
            components.insert(name, Box::new(component));
        } else if line.starts_with("&") {
            let (name, component) = Conjunction::from_str(line);
            conjunctions.insert(name, component);
        } else {
            let (name, component) = FlipFlop::from_str(line);
            components.insert(name, Box::new(component));
        };
        //components.insert(name, component);
    }
    // Now, for each conjunction node we need to find its inputs
    for (node_name, node) in components.iter() {
        for destination in node.destinations() {
            if let Some(e) = conjunctions.get_mut(destination) {
                e.recent_pulse.insert(node_name.clone(), false);
            }
        }
    }
    for (name, node) in conjunctions {
        components.insert(name, Box::new(node));
    }
    components
}

pub fn solve() {
    let mut file = File::open("inputs/day_20.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut components = parse_content(&contents);
    components.insert(String::from("output"), Box::new(Output::new()));
    //println!("My components are {:#?}\n", components);
    // source, destination, pulse
    let n_pushes = 1000;
    let mut low_pulses = n_pushes;
    let mut high_pulses = 0;
    for _ in 0..n_pushes {
        let mut pulses: VecDeque<(String, String, bool)> = VecDeque::from([(String::from("button"), String::from("broadcaster"), false)]);
        while pulses.len() > 0 {
            let current_pulse = pulses.pop_front().unwrap();
            if let Some(new_pulses) = components.entry(current_pulse.1.clone()).or_insert(Box::new(Output::new())).signal(&(current_pulse.0.clone(), current_pulse.2)) {
                for new_pulse in new_pulses {
                    //println!("{} {}-> {} ", current_pulse.1, new_pulse.1, new_pulse.0);
                    pulses.push_back((current_pulse.1.clone(), new_pulse.0, new_pulse.1));
                    if new_pulse.1 {
                        high_pulses += 1;
                    } else {
                        low_pulses += 1;
                    }
                }
            }
        }
    }
    println!("low pulses {}\nhigh pulses {}\ntotal {}", low_pulses, high_pulses, low_pulses * high_pulses);
    //println!("\nMy components are {:#?}", components);
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_20.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut components = parse_content(&contents);
    components.insert(String::from("output"), Box::new(Output::new()));
    //println!("My components are {:#?}\n", components);
    // source, destination, pulse
    let mut n_presses = 0;
    let mut go = true;
    while go {
        n_presses += 1;
        let mut pulses: VecDeque<(String, String, bool)> = VecDeque::from([(String::from("button"), String::from("broadcaster"), false)]);
        while pulses.len() > 0 {
            let current_pulse = pulses.pop_front().unwrap();
            if let Some(new_pulses) = components.entry(current_pulse.1.clone()).or_insert(Box::new(Output::new())).signal(&(current_pulse.0.clone(), current_pulse.2)) {
                for new_pulse in new_pulses {
                    if !new_pulse.1 & (new_pulse.0 == String::from("rx")) {
                        go = false;
                    }
                    pulses.push_back((current_pulse.1.clone(), new_pulse.0, new_pulse.1));
                }
            }
        }
    }
    println!("Number of presses {n_presses}");
}