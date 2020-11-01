use std::env;
use std::fs;
use std::collections::HashMap;
struct Config {
    fname: String,
    mode: i32,
}
impl Config{
	fn new(args:&[String])->Config{
		let fname = &args[1].clone();
		let mode = &args[2].clone();	
		Config { fname:fname.to_string(), mode:mode.to_string().parse::<i32>().unwrap()}
	}

}
struct Chain {
	p:i32,
}
impl Chain {
	 fn go(&self)->i32{
	 	self.p
	 }
}
#[allow(dead_code)]
struct Word<'a> {
	word:&'a str,
	freq:u32,
}
impl Word<'_> {
	#[allow(dead_code)]
	fn new(word:&str)->Word{
		Word{word:word,freq:1}
	}
}
struct Principle<'a>{
	word: &'a str,
	next: Vec<&'a str>,
}

#[allow(dead_code)]
fn amke_word() -> &'static str{
	"wordddd"
}



// 
fn main() {
	let ch :Chain = Chain{p:22};
	ch.go();
	let pr = Principle{word:"",next:vec!["dd"]};
	println!("Voila! {:?} -> {}",pr.word,pr.next[0]);

 	let args: Vec<String> = env::args().collect();
 	let config = Config::new(&args);
 	
 	
 	println!("Mode is {:?} ",config.mode );


 	let mut uniqs : HashMap<&str,u32> = HashMap::new();
	let res = split_uniq("big.txt", &mut uniqs,true);
 	
 	
 	
	// println!("{:?} ", uniqs);
	// println!("Words are {:?} ",wh );
 	// println!("Uniq words are {:?} ",uniq );
 	// println!("Words: {:?} ",words.next() );
 	// println!("Uniq:  {:?} ",uniq.len() );


}

fn split_uniq(fname:&str,uniqs:&mut HashMap<&str,u32>,verbose:bool)->(){
	let fname = fname.to_string();
	let content = fs::read_to_string(fname)
		.expect("Error reading file");
	let  words : std::str::SplitWhitespace = content.split_whitespace();
	if verbose {
		println!("{:?} has content {:?} length",fname.to_string(),content.len() );
	}
	for word in words {
		match (&uniqs).get(word){
			None=> uniqs.insert(word,1),
			Some(num)=>uniqs.insert(word,num+1)
		};
	}
	for w in uniqs{
		match w {
			(_,0_u32) 				=> println!("{:?} {:?} zero ",w.0,w.1),
			(_,1_u32) 				=> (),
			(_,2_u32..=100)		 	=> (),
			(_,100_u32..=u32::MAX)	=> println!("{:?} {:?} big ",w.0,w.1),
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_word() {
        assert_eq!("wordddd", amke_word());
    }
    // fn splits_text() {
    //     assert_eq!(["qucik brown fox jump over lazy dog, blue cunning frog,manage stuf to smoke"], bigger(10, 8));
    // }

}