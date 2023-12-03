use std::collections::HashMap;

struct Game {
    id: u32,
    cubes: HashMap<String, u32>,
}

pub fn solve_first(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let splitted_line = line.split(": ").collect::<Vec<&str>>();
            let game_id = splitted_line[0].split(' ').collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();
            let subset_cubes: Vec<Vec<(&str, u32)>> = splitted_line[1]
                .split("; ")
                .map(|subset| {
                    subset
                        .split(", ")
                        .map(|subsubset| {
                            let splitted = subsubset.split(' ').collect::<Vec<&str>>();
                            (splitted[1], splitted[0].parse::<u32>().unwrap())
                        })
                        .collect::<Vec<(&str, u32)>>()
                })
                .collect::<Vec<Vec<(&str, u32)>>>();
            let mut cubes: HashMap<String, u32> = HashMap::new();
            for subset in subset_cubes {
                for (cube, cube_count) in subset {
                    if let Some(c) = cubes.get(cube) {
                        if cube_count > *c {
                            cubes.insert(cube.to_string(), cube_count);
                        }
                    } else {
                        cubes.insert(cube.to_string(), cube_count);
                    }
                }
            }
            Game { id: game_id, cubes }
        })
        .filter(|game| {
            if let Some(red) = game.cubes.get("red") {
                if *red > 12 {
                    return false;
                }
            } else {
                return false;
            }
            if let Some(green) = game.cubes.get("green") {
                if *green > 13 {
                    return false;
                }
            } else {
                return false;
            }
            if let Some(blue) = game.cubes.get("blue") {
                if *blue > 14 {
                    return false;
                }
            } else {
                return false;
            }
            true
        })
        .map(|game| game.id)
        .sum::<u32>()
        .to_string()
}

pub fn solve_second(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let splitted_line = line.split(": ").collect::<Vec<&str>>();
            let game_id = splitted_line[0].split(' ').collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();
            let subset_cubes: Vec<Vec<(&str, u32)>> = splitted_line[1]
                .split("; ")
                .map(|subset| {
                    subset
                        .split(", ")
                        .map(|subsubset| {
                            let splitted = subsubset.split(' ').collect::<Vec<&str>>();
                            (splitted[1], splitted[0].parse::<u32>().unwrap())
                        })
                        .collect::<Vec<(&str, u32)>>()
                })
                .collect::<Vec<Vec<(&str, u32)>>>();
            let mut cubes: HashMap<String, u32> = HashMap::new();
            for subset in subset_cubes {
                for (cube, cube_count) in subset {
                    if let Some(c) = cubes.get(cube) {
                        if cube_count > *c {
                            cubes.insert(cube.to_string(), cube_count);
                        }
                    } else {
                        cubes.insert(cube.to_string(), cube_count);
                    }
                }
            }
            Game { id: game_id, cubes }
        })
        .map(|game| {
            let red = game.cubes.get("red").unwrap();
            let green = game.cubes.get("green").unwrap();
            let blue = game.cubes.get("blue").unwrap();
            *red * *green * *blue
        })
        .sum::<u32>()
        .to_string()
}
