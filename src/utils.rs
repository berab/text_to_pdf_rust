use std::fs;
use std::path;

pub struct Exercise {
    words: Vec<String>,
    sentences: Vec<String>,
}


pub fn test_file_to_exercise() -> Exercise {
    let file_path = path::Path::new("./exercises/exercise_for_testing.txt");
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file.");
    

    let mut exercise = Exercise {
        words: Vec::new(),
        sentences: Vec::new(),
    };
    
    for line in  contents.lines() {
        let first_char = line.chars().nth(0);
        let second_char = line.chars().nth(1);

        match (first_char, second_char) {
            (Some(c1), Some(c2)) => {
                if c1 == '-' {
                    exercise.words.push(line[2..].to_string()); // lines[2..] after first two indices
                } else if c2 == '.' {
                    exercise.sentences.push(line.to_string());
                }
            },
            (_, _) => (),
        }
    }

    exercise
}

#[cfg(test)]
mod tests {
    use super::{test_file_to_exercise, Exercise};
    
    #[test]
    fn test_test_file_to_exercise() {
        let res_exercise: Exercise = Exercise {
            words: vec![
                String::from("word1"),
                String::from("word2"),
                String::from("word3"),
                ],
            sentences: vec![
                String::from("1. __________, fill this please."),
                String::from("2. __________ and this?"),
                String::from("3. Of course, and this __________?"),
                ],
        };

       let exercise: Exercise = test_file_to_exercise();
       for i in 0..3 {
        assert!(exercise.words[i] == res_exercise.words[i]);
        assert!(exercise.sentences[i] == res_exercise.sentences[i]);
       }

    }
}
