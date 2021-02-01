fn main() {
  demo_vector();
  element_of_vector();
  iterating_vector();
  demo_string();
  add_string();
  add_string2();
  unicode_string();
  demo_hashmap();
  count_text();
}

fn demo_vector() {
  let mut v = Vec::new();
  v.push(1);
  v.push(2);
  v.push(9);
  v.push(10);
  println!("v = {:?}", v);
}

fn element_of_vector() {
  let mut v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];

  // v.push(6); This will cause panic

  println!("Third element = {}", third);

  match v.get(12) {
    Some(third) => println!("Thirteenth element = {}", third),
    None => println!("There is no Thirteenth element."),
  }

}

fn iterating_vector() {
  let mut v = vec![100, 32, 34];
  for i in &v {
    println!("{}", i);
  }

  for i in &mut v {
    *i += 50;
  }

  println!("New value {:?}", v);

}

fn demo_string() {
  let mut s = String::from("foo");
  let s2 = "bar";

  s.push_str(s2);

  println!("s: {}", s);
  println!("s2: {}", s2);
}

fn add_string() {
  let s1 = "tic".to_string();
  let s2 = "tac".to_string();
  let s3 = "toe".to_string();

  let s = s1 + "-" + &s2 + "-" + &s3;

  println!("{}", s);

}

fn add_string2() {
  let s1 = "tic".to_string();
  let s2 = "tac".to_string();
  let s3 = "toe".to_string();

  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("{}", s)
}

fn unicode_string() {
  let hello = String::from("Здравствуйте");
  println!("len: {}", hello.len());

  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  println!("Bytes:");
  for c in "नमस्ते".bytes() {
    println!("{}", c);
  }
}

use std::collections::HashMap;
fn demo_hashmap() {
  let team = vec![String::from("Blue"), String::from("Red")];
  let scores = vec![10, 50];
  let mut scores_team: HashMap<_, _> = team.into_iter().zip(scores.into_iter()).collect();

  println!("scores_team: {:?}", scores_team);

  scores_team.insert(String::from("Blue"), 25);
  println!("Blue score: {:?}", scores_team.get(&String::from("Blue")));

  scores_team.entry(String::from("Yellow")).or_insert(1000);
  scores_team.entry(String::from("Yellow")).or_insert(2000);
  println!("scores_team: {:?}", scores_team);
}

fn count_text() {
  let text = "hello world wonderful world";
  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("Word count: {:?}", map);
}