fn main() {
    let input_one = include_str!("../../../2024/inputs/quest_01/part_01.txt");
    
    let answer_one = part_one(input_one);
    
    println!("Part one: {}\n", answer_one);
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let input = "ABBAC";
        let answer = 5;
        
        assert_eq!(part_one(input), answer);
    }
}
