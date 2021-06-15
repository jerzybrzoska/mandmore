use std::{fs, io};


pub fn ih_correct() -> String {
//pub fn ih_correct() { //error[E0308]: mismatched types 
 println!("Give me the input hash. The input hash has to have exactly 40 characters.");

 let mut ih = String::new();
 
 io::stdin().read_line(&mut ih).expect("Failed to read line"); 
  	
 ih = ih.trim_end().to_string();

 if ih.chars().count() != 40 {
      // ih_correct(); //when defined  -> String  then: expected struct `String`, found `()`  - help: consider removing this semicolon
        ih_correct() //
        }
 else {
	 println!("Give me the name for the content of this magnet link.");
	 let mut dn = String::new();
 
     io::stdin()
        .read_line(&mut dn)
        .expect("Failed to read line"); 
    if dn.trim_end().chars().count() < 2 {
		
	 dn = "".to_string(); 
    }
    else {
		
	let prefix = "&dn=";
    
    dn = format!("{}{}", prefix, dn);
    }
    println!("Key terms?");    
    let mut kt = String::new();
 
     io::stdin()
        .read_line(&mut kt)
        .expect("Failed to read line");
        
    if kt.chars().count() < 3 {
		
	 kt = "".to_string(); 
    }
    else {
		
	let prefix = "&kt=";
    
    kt = format!("{}{}", prefix, kt);
	 
    	 
  }
  
  println!("Acceptable source?");    
    let mut acceptable = String::new();
 
     io::stdin()
        .read_line(&mut acceptable)
        .expect("Failed to read line");
        
    if acceptable.chars().count() < 5 {
		
	 acceptable = "".to_string(); 
    }
    else {
		
	let prefix = "&as=";
    
    acceptable = format!("{}{}", prefix, acceptable);
	 	 
  }
  
//let tsm = r##"tsm -a "magnet:?xt=urn:btih:"##;//This gives you "tsm -a \"magne....
  let tsm = r#"transmission-remote -a "magnet:?xt=urn:btih:"#;//
  format!("{}{}{}{}{}", tsm, ih, dn.trim(), kt.trim(), acceptable.trim()) //This gives you "tsm -a \"magne....
  
 }
}

//Issues:
