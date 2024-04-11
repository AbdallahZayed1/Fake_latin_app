use std::io;
fn main() {
    let mut word = String::new();
 println!("Enter a word to convert it into latino");
io::stdin().read_line(&mut word).expect("leh enta keda");
latino(&word.trim());
}
fn latino(word : &str) {
    let vowel: [char; 10]  = ['a','o','u','i','e','A','O','U','I','E'];
    let mut bytes = word.chars();

    let first_letter = &bytes.next();

    match first_letter {
        Some(t) => 
        {
            fn get_letter(letter : &char , vowel: [char; 10], word: &str) -> String{
                let other_letters = &word[1..] ;
                let mut latin_word = String::new();
                for volo in vowel {
                    if letter == &volo{
                          latin_word = format!("{}-hay",word);
                        
                     break ;
                    } 
                    else {
                         latin_word = format!("{}-{}ay",other_letters,letter);
                    }
                    
            }latin_word

            }
            let latino = get_letter(t, vowel, word);
            println!("The word in latino is : {}",latino.to_ascii_uppercase());
        }
        ,
        None => println!("enter a word yalllla")
    }
    
}

