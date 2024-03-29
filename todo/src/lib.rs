pub mod user_input;
pub fn split_line(text: String) -> Vec<String>{
	let mut output: Vec<String> = Vec::new();
   	let mut flag = false;
   	let mut temp: String = String::new();
   	for i in text.chars(){
       	if i == ']' && flag{
           	flag = false;
           	output.push(temp);
           	temp = String::new();
       	}
       	else if i == '[' && !flag{
           	flag = true;
       	}
       	else if flag {
           	temp.push(i);
       	}
   	}
 	output
}
