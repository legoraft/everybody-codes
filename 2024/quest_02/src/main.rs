fn main() {
    let input_one = include_str!("../../inputs/quest_02/part_01.txt");
//    let input_two = include_str!("../../inputs/quest_02/part_02.txt");
//    let input_three = include_str!("../../inputs/quest_02/part_03.txt");
    
    let answer_one = part_one(input_one);
    let answer_two = 0;
    let answer_three = 0;
    
    println!("Part one: {}\nPart two: {}\nPart three: {}", answer_one, answer_two, answer_three);
}

fn part_one(input: &str) -> i64 {
    let (words, inscription) = input.split_once("\n\n").unwrap();
    let mut count = 0;
    
    let (_, words) = words.split_once(":").unwrap();
    let words: Vec<&str> = words.split(",").collect();
    let inscription: Vec<&str> = inscription.split_whitespace().collect();
    
    for word in inscription {
        if words.iter().any(|w| word.contains(w)) {
            dbg!(&word);
            count += 1;
        }
    }
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        let input = "\
WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE THERE";
        let answer = 4;
        
        assert_eq!(part_one(input), answer);
    }
}