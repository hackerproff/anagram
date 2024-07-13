/*
https://contest.yandex.ru/contest/8458/problems/E/
*/

fn main() {

    let mut readed_string_1 = String::new();
    let mut readed_string_2 = String::new();

    std::io::stdin().read_line(&mut readed_string_1).expect("0");
            
    if  readed_string_1.len() == 0 { println!("0"); return; }
    
    if !readed_string_1.is_ascii() { println!("0"); return; }
    
    std::io::stdin().read_line(&mut readed_string_2).expect("0");
    
    if  readed_string_2.len() == 0 { println!("0"); return; }
    
    if !readed_string_2.is_ascii() { println!("0"); return; }
    
	let mut str_vec1 = readed_string_1.into_bytes();
	
	str_vec1.sort();
   
	let mut str_vec2 = readed_string_2.into_bytes();
	
	str_vec2.sort();

    if   str_vec2 == str_vec1 {
    
    	println!("1");
    } else {
    	println!("0");
    }
}
