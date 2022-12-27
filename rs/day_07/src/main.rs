use num::abs;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct System {
    root: Directory,
    current_path: String,
}

impl System {
    fn new() -> System {
        System {
            root: Directory {
                name: String::from("/"),
                files: vec![],
                subdirectories: vec![],
            },
            current_path: String::from(""),
        }
    }

    fn cd(&mut self, target: &str) {
        if target == ".." {
            let mut path = self.current_path.split("/").collect::<Vec<&str>>();
            path.pop();
            self.current_path = path.join("/");
        } else if target == "/" {
            self.current_path = String::from("");
        } else {
            self.current_path = format!("{}/{}", self.current_path, target);
        }
    }

    fn mkdir(&mut self, name: &str) {
        let path = self.current_path.split("/")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let mut current_dir = &mut self.root;
        for dir in path {
            current_dir = current_dir.subdirectories.iter_mut().find(|d| d.name == dir).unwrap();
        }
        current_dir.subdirectories.push(Directory {
            name: String::from(name),
            files: vec![],
            subdirectories: vec![],
        });
    }

    fn touch(&mut self, name: &str, size: u64) {
        let path = self.current_path.split("/")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let mut current_dir = &mut self.root;
        for dir in path {
            current_dir = current_dir.subdirectories.iter_mut().find(|d| d.name == dir).unwrap();
        }
        current_dir.files.push(RawFile {
            name: String::from(name),
            size,
        });
    }
}

#[derive(Debug)]
struct RawFile {
    name: String,
    size: u64,
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<RawFile>,
    subdirectories: Vec<Directory>,
}

impl Directory {
    fn get_size(&self) -> u64 {
        let mut size = 0;
        for file in &self.files {
            size += file.size;
        }
        for dir in &self.subdirectories {
            size += dir.get_size();
        }
        size
    }
}

fn part1(directories: &Vec<Directory>, acc: u64) -> u64 {
    let mut total = acc;
    for dir in directories {
        if dir.get_size() < 100000 {
            println!("{}: {}", dir.name, dir.get_size());
            total += dir.get_size();
        }
        total = part1(&dir.subdirectories, total);
    }
    total
}

fn handle_cmd(sys: &mut System, cmd: &str) {
    if cmd.chars().nth(0) == Some('$') {
        let re = Regex::new(r"\$ (\w+)(?: (.*))?").unwrap();
        if let Some(captures) = re.captures(cmd) {
            let command = &captures[1];
            if command == "cd" {
                sys.cd(&captures[2])
            } else if command == "ls" {

            }
        }
    } else {
        let split = cmd.split(" ").collect::<Vec<&str>>();
        if split[0] == "dir" {
            sys.mkdir(split[1])
        } else {
            sys.touch(split[1], split[0].parse::<u64>().unwrap())
        }
    }
}

fn part2(sys: &System) -> u64 {
    let MAX_SPACE_AVAILABLE = 70000000;
    let SPACE_NEEDED_FOR_UPDATE = 30000000;
    let current_size_of_sys = sys.root.get_size();
    let size_needed_still = SPACE_NEEDED_FOR_UPDATE - (MAX_SPACE_AVAILABLE - current_size_of_sys);

    println!("target: {}", size_needed_still);
    find_closest_size_to_target(&sys.root.subdirectories, size_needed_still, current_size_of_sys)
}

fn find_closest_size_to_target(directories: &Vec<Directory>, target: u64, closest: u64) -> u64 {
    let mut closest = closest;
    for dir in directories {
        let dir_size = dir.get_size();
        // println!("checking dir {} with size {}", dir.name, dir_size);
        let a = dir_size as i64 - target as i64;
        let b = closest as i64 - target as i64;
        // println!("a: {}, b: {}", a, b);
        if (dir_size > target) &&  a < b {
            closest = dir_size;
            // println!("found new closest dir {} with size {}", dir.name, closest)
        }
        closest = find_closest_size_to_target(&dir.subdirectories, target, closest);
    }
    closest
}

fn main() {
    let mut sys = System::new();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        handle_cmd(&mut sys, &line);
    }

    // let p1 = part1(&sys.root.subdirectories, 0);
    // println!("Part 1: {}", p1)

    let p2 = part2(&sys);
    println!("Part 2: {}", p2)
}