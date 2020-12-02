use itertools::Itertools;

pub fn day2a(lines: &mut Vec<String>) -> u32 {
    let mut matches = 0u32;
    for line in lines {
        let vec = line.split(": ").collect::<Vec<_>>();
        let password = vec[1];
        let policy = vec[0].split(" ").collect::<Vec<_>>();

        let letter = policy[1];
        let range : (usize,usize) = policy[0].split("-").map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();

        let count = password.matches(letter).count();

        if count >= range.0 && count <= range.1 {
            matches += 1;
        }
    }

    matches
}

pub fn day2b(lines: &mut Vec<String>) -> u32 {
    let mut matches = 0u32;

    for line in lines {
        let vec = line.split(": ").collect::<Vec<_>>();
        let password = vec[1];
        let policy = vec[0].split(" ").collect::<Vec<_>>();

        let letter = policy[1].chars().nth(0).unwrap();
        let range : (usize,usize) = policy[0].split("-").map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();

        let chars = password.chars().collect::<Vec<char>>();

        if (chars[range.0 - 1] == letter) ^ (chars[range.1 - 1] == letter)  {
            matches += 1;
        }
    }

    matches
}