fn main() {
    let input_one = include_str!("../../inputs/quest_01/part_01.txt");
    let input_two = include_str!("../../inputs/quest_01/part_02.txt");
    let input_three = include_str!("../../inputs/quest_01/part_03.txt");
    
    let answer_one = part_one(input_one);
    let answer_two = part_two(input_two);
    let answer_three = part_three(input_three);
    
    println!("Part one: {}\nPart two: {}\nPart three: {}", answer_one, answer_two, answer_three);
}

fn part_one(input: &str) -> i64 {
    let creatures: Vec<char> = input.chars().collect();
    let mut answer: i64 = 0;
    
    for creature in creatures {
        match creature {
            'A' => answer += 0,
            'B' => answer += 1,
            'C' => answer += 3,
            _ => panic!("That can't happen!")
        }
    }
    
    answer
}

fn part_two(input: &str) -> i64 {
    let creature_pairs: Vec<(char, char)> = input
        .chars().step_by(2)
        .zip(input.chars().skip(1).step_by(2))
        .collect();
    
    let mut answer = 0;
    
    for pair in creature_pairs {
        let mut points = 0;
        
        match pair.0 {
            'A' => points += 1,
            'B' => points += 2,
            'C' => points += 4,
            'D' => points += 6,
            'x' => points -= 1,
            _ => panic!("That can't happen!")
        }
        
        match pair.1 {
            'A' => points += 1,
            'B' => points += 2,
            'C' => points += 4,
            'D' => points += 6,
            'x' => points -= 1,
            _ => panic!("That can't happen!")
        }
        
        if points < 0 {
            points = 0;
        }
        
        answer += points;
    }
    
    answer
}

fn part_three (input: &str) -> i64 {
    let characters: Vec<char> = input.chars().collect();
    let creature_pairs: Vec<&[char]> = characters.chunks(3).collect();
    
    let mut answer = 0;
    
    for pair in creature_pairs {
        let mut potions: i64 = 0;
        let empty = pair.iter().filter(|c| *c == &'x').count();
        
        for creature in pair {
            match creature {
                'A' => potions += 2 - empty as i64,
                'B' => potions += 3 - empty as i64,
                'C' => potions += 5 - empty as i64,
                'D' => potions += 7 - empty as i64,
                'x' => continue,
                _ => panic!("That can't happen!")
            }
        }
        
        if potions < 0 {
            continue;
        }
        
        answer += potions;
    }
    
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let input = "ABBAC";
        let answer = 5;
        
        assert_eq!(part_one(input), answer);
    }
    
    #[test]
    fn test_part_two() {
        let input = "AxBCDDCAxD";
        let answer = 28;
        
        assert_eq!(part_two(input), answer);
    }
    
    #[test]
    fn test_part_three() {
        let input = "xBxAAABCDxCC";
        let answer = 30;
        
        assert_eq!(part_three(input), answer);
    }
}
