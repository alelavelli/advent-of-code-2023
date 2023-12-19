use std::{fs::File, io::Read, collections::HashMap, fmt::Debug};


struct Rule {
    part_name: Option<String>,
    condition: Box<dyn Fn(&Part) -> bool>,
    destination: String
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule").field("destination", &self.destination).finish()
    }
}

impl Rule {
    fn from_str(value: &str) -> Rule {
        let split: Vec<&str> = value.split(":").collect();
        if split.len() > 1 {
            let destination = String::from(*split.get(1).unwrap());
            let rule = *split.get(0).unwrap();
            let (part_name, condition): (String, Box<dyn Fn(&Part) -> bool>) = if rule.contains(">") {
                let rule_parts = rule.split(">").collect::<Vec<&str>>();
                let part_name = String::from(*rule_parts.get(0).unwrap());
                let part_number = rule_parts.get(1).unwrap().parse::<i32>().unwrap();
                (
                    String::from(part_name.clone()),
                    Box::new( move |x: &Part| x.content.get(&part_name).map_or(false, |v| *v > part_number))
                )
            } else {
                let rule_parts = rule.split("<").collect::<Vec<&str>>();
                let part_name = String::from(*rule_parts.get(0).unwrap());
                let part_number = rule_parts.get(1).unwrap().parse::<i32>().unwrap();
                (
                    String::from(part_name.clone()),
                    Box::new(move |x: &Part| x.content.get(&part_name).map_or(false, |v| *v < part_number))
                )
            };
            Rule { part_name: Some(part_name), condition, destination }
        } else {
            // in case of single destination rfg
            let destination = String::from(*split.get(0).unwrap());
            Rule { part_name: None, condition: Box::new(|_: &Part| true ), destination }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>
}

impl Workflow {
    fn from_str(value: &str) -> Workflow {
        let replaced = value.replace("}", "");
        let split = replaced.split("{").collect::<Vec<&str>>();
        let name = String::from(*split.get(0).unwrap());
        let mut rules: Vec<Rule> = Vec::new();
        for rule_str in split.get(1).unwrap().split(",") {
            rules.push(Rule::from_str(rule_str));
        }

        Workflow { name, rules }
    }

    fn get_destination(&self, part: &Part) -> &str {
        for rule in self.rules.iter() {
            let result = rule.condition.as_ref()(part);
            if result {
                return &rule.destination
            }   
        }
        panic!("ops no rules matched");
    }
}

#[derive(Debug)]
struct Part {
    content: HashMap<String, i32>
}

impl Part {
    fn from_str(value: &str) -> Part {
        let mut parsed = value.replace("(", "");
        parsed = parsed.replace(")", "");
        parsed = parsed.replace("}", "");
        parsed = parsed.replace("{", "");
        let parsed = parsed.split(",").collect::<Vec<&str>>();
        let mut content: HashMap<String, i32> = HashMap::new();

        for elem in parsed {
            let split = elem.split("=").collect::<Vec<&str>>();
            content.insert(String::from(*split.get(0).unwrap()), split.get(1).unwrap().parse::<i32>().unwrap());
        }

        Part { content }
    }
}


fn parse_contents(contents: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts : Vec<Part> = Vec::new();

    let mut building_workflows = true;

    for line in contents.lines() {
        if line.len() == 0 {
            building_workflows = false;
            continue;
        }

        if building_workflows {
            let workflow = Workflow::from_str(line);
            workflows.insert(workflow.name.clone(), workflow);
        } else {
            let part = Part::from_str(line);
            parts.push(part);
        }
    }
    (workflows, parts)
}

pub fn solve() {
    let mut file = File::open("inputs/day_19.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let (workflows, parts) = parse_contents(&contents);
    println!("Workflows {:#?}", workflows);
    println!("Parts {:#?}", parts);

    let mut accepted_parts: Vec<&Part> = Vec::new();

    
    for part in parts.iter() {
        let mut workflow = workflows.get(&String::from("in")).unwrap();
        let mut destination = workflow.get_destination(&part);

        while (destination != "A") & (destination != "R") {
            workflow = workflows.get(&String::from(destination)).unwrap();
            destination = workflow.get_destination(part);
        }

        if destination == "A" {
            accepted_parts.push(part);
        }
    }
    let mut result = 0;
    for part in accepted_parts {
        for content in part.content.values() {
            result += content;
        }
    }
    println!("Result is {}", result);
}


pub fn solve_pt2() {
    let mut file = File::open("inputs/day_19_example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let (workflows, _) = parse_contents(&contents);
    
}
