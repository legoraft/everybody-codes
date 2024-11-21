fn main() {
    let input_one = include_str!("../../inputs/quest_01/part_01.txt");
    let input_two = include_str!("../../inputs/quest_01/part_01.txt");
    
    let answer_one = part_one(input_one);
    let answer_two = part_two(input_two);
    
    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
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
        
        answer += points;
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
}
