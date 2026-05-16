use std::{collections::HashMap};


fn bpe_tokenize(input: &str) -> HashMap<String, (String, String)>
{
    let mut encoded_string: Vec<String>= Vec::new();
    let mut vocabulary = HashMap::new();

    for c in input.chars(){
        encoded_string.push(c.to_string());
    }
    encoded_string.push("".to_string());
    println!("{:?}", encoded_string);

    for _ in 0..4 {

        let mut pairs:HashMap<(String, String), u64> = HashMap::new();
        for i in 0..encoded_string.len() - 1{

            let s1: &str = &encoded_string[i];
            let s2: &str = &encoded_string[i + 1];

            *pairs.entry((s1.to_string(), s2.to_string())).or_insert(0) += 1;


        }
        let best_pair = pairs.iter().max_by_key(|(_, v)| **v).map(|(k, _)| k).unwrap();
        println!("best pair {:?} + {:?}",best_pair.0, best_pair.1);
        let merged_pair = format!("{}{}", best_pair.0, best_pair.1);

        let mut new_encoded_string: Vec<String>= Vec::new();

        let mut i = 0;
        while  i  < encoded_string.len(){
            if encoded_string[i] == best_pair.0 && encoded_string[i + 1] == best_pair.1
                && i < encoded_string.len() - 1{
                    new_encoded_string.push(merged_pair.clone());
                    i += 1;
            } else {
                new_encoded_string.push(encoded_string[i].clone());
            }
            i += 1;
        }
        encoded_string = new_encoded_string;
        vocabulary.insert(merged_pair, (best_pair.0.to_string(), best_pair.1.to_string()));
        println!("{:?}", encoded_string);
    }


    vocabulary
}

fn main() {
    let v = bpe_tokenize("low low low lower lower lower");
    println!("{:#?}", v);
}
