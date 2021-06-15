//We want this file to create a magnet link, write it to an Bash script and execute the said script
use std::{fs, fs::File, fs::OpenOptions, io::prelude::*, path::Path, io::Write, process, process::Command};
use home;
use error_chain::error_chain;
use crate::ih_correct::ih_correct;
mod ih_correct;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
async fn main() -> Result<()> {
  	
	
 let mut home = String::new();

 home = home::home_dir().unwrap().into_os_string().into_string().unwrap();
 
 let path = format!("{}/.mandmore", home);
 
 if Path::new(&path).exists() {
 
  //  println!("{} exists.", path);
 }
 else {
  println!("{} does not exist - methinks this app is being run for the first time on this OS. I will create {} and then exit. Please run the app again.", path, path);
  let mut tsmadd = Command::new("/usr/bin/mkdir");
 
    tsmadd.arg(path).spawn();
    process::exit(0);
	
 }
 let donations = "\n\nDonations would help me to continue developing this project:\n\nMonero (XMR):\n\n89v8u8tFSsA1iYHgHYvtypBpbQLei7hZ73ajSDQvbsaX2h2213d74rvTwGueRV8YEphkGLiWqo1fD1xXTVcni4iMMCGRvj8\n\n";

 let name = "magnet_links_history";
 
 let path = format!("{}/.mandmore/{}", home, name);	

 let mut combined = String::new();
	
	let target = "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_best.txt";
    let response = reqwest::get(target).await?;

    let content =  response.text().await?;
    //Making a String from Option<PathBuf>
      
    let name_trackers = "trackers_best.txt";
 
    let path_trackers = format!("{}/.mandmore/{}", home, name_trackers);

    let mut file = File::create(&path_trackers).unwrap();
    file.write_all(content.as_bytes()).unwrap(); 
        
    let mut contents = fs::read_to_string(&path_trackers).expect("Something went wrong reading the file");
    contents = contents.replace("\n\n", "&tr="); //This leaves me with "&tr=" to be trimmed from the end and no "&tr=" at the beginning
    
    if contents.ends_with("&tr=") {
      contents = (&contents[..contents.len() - "&tr=".len()]).to_string()
      }
    let prefix = "&tr=";
    
    combined = format!("{}{}", prefix, contents);
    
    let to_download = format!("{}{}\"", ih_correct(), combined);
    
 
 let path = format!("{}/.mandmore/{}", home, name);
        
 let mut file = OpenOptions::new()
   .create(true)
   .write(true)
   .append(true)
   .open(path) //For the the ease of navigation: 0 12 * * * /usr/bin/date +"\%H:\%M \%d/\%m/\%Y">>/home/jerzy/magnet_links_history
   .unwrap();
 
    writeln!(file, "{}", to_download); //error[E0382]: borrow of moved value: `to_download` 
    writeln!(file, "{}", donations); //error[E0382]: borrow of moved value: `to_download` 
    
    let name_of_script = "tsmadd.sh";
 
    let path_of_script = format!("{}/.mandmore/{}", home, name_of_script);

   
 //   fs::write(path_of_script, to_download); // error[E0382]: use of moved value: `path_of_script`
    fs::write(&path_of_script, to_download); // 

    let mut tsmadd = Command::new("bash");
 
    tsmadd.arg(path_of_script);
     
    tsmadd.output().expect("failed to execute process");
    
    

   /*  if let Err(e) = writeln!(file, "{}", to_download) { //move occurs because `to_download` has type `String`, which does not implement the `Copy` trait
     eprintln!("Couldn't write to file: {}", e);
  }*/

    Ok(())
     
}

/* Issues
How do I append to a file in Rust?
transmission-remote -a "magnet:?xt=urn:btih:358CBFDAC5840426C8832B6A731A61511F9E2EDB&dn=%2B%2B%2B+FC2-PPV-1217012+%5Bcompletely+Amateur+62%E3%80%91in+the+age+of+18+and+of+2%2C+full+face%2C+the+cum+shot%26amp%3Bcum+2+barrage%2C+anal+plug+insertion&tr=http%3A%2F%2Fsukebei.tracker.wf%3A8888%2Fannounce&tr=http%3A%2F%2Ft.nyaatracker.com%3A80%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=http%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fopentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fbt.xxx-tracker.com%3A2710%2Fannounce&tr=udp%3A%2F%2Fexplodie.org%3A6969%2Fannounce&tr=udp%3A%2F%2F62.138.0.158%3A6969%2Fannounce&tr=http%3A%2F%2Fanidex.moe%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.pirateparty.gr%3A6969&tr=udp%3A%2F%2Ftracker.zer0day.to%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969%2Fannounce&tr=udp%3A%2F%2Fcoppersurfer.tk%3A6969%2Fannounce"
transmission-remote -a "magnet:?xt=urn:btih:77BAA53C39525A12F9B0F007A84EBC58ECF8680D&dn=Triple+Penetration+%282020%29%5BWEBRip%5D%5B.MP4%5D&tr=http%3A%2F%2Ftracker2.itzmx.com%3A6961%2Fannounce&tr=http%3A%2F%2Fvps02.net.orel.ru%3A80%2Fannounce&tr=http%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2F9.rarbg.com%3A2710%2Fannounce&tr=udp%3A%2F%2F9.rarbg.me%3A2710&tr=udp%3A%2F%2Fopen.demonii.si%3A1337%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.zer0day.to%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969%2Fannounce&tr=udp%3A%2F%2Fcoppersurfer.tk%3A6969%2Fannounce"
transmission-remote -a "magnet:?xt=urn:btih:69A6D09D7BE2DA8E99618BA8FD3583603BB12E2C&dn=LegalPorno+-+10on1+Triple+anal+Gangbang+with+Anna+de+Ville+with+Balls+Deep+Anal%2C+DAP%2C+TAP%2C+Big+Gapes%2C+Facial&tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&tr=udp%3A%2F%2F9.rarbg.me%3A2710%2Fannounce&tr=udp%3A%2F%2F9.rarbg.to%3A2710%2Fannounce&tr=udp%3A%2F%2F9.rarbg.to%3A2710%2Fannounce&tr=udp%3A%2F%2Feddie4.nl%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.pirateparty.gr%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969%2Fannounce&tr=udp%3A%2F%2Fshadowshq.yi.org%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2Fp4p.arenabg.com%3A1337%2Fannounce&tr=udp%3A%2F%2Fp4p.arenabg.ch%3A1337%2Fannounce&tr=udp%3A%2F%2F62.138.0.158%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.zer0day.to%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.zer0day.to%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969%2Fannounce&tr=udp%3A%2F%2Fcoppersurfer.tk%3A6969%2Fannounce"
transmission-remote -a "magnet:?xt=urn:btih:42148EE8E79F1186C6E3ECF4BC5563B4AEC4B5D7&dn=%5BAmateur+Porn%5D+Sister+Wanted+my+Tongue+Deep+in+her+Asshole+720p+gt&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Fdenis.stalker.upeer.me%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.port443.xyz%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.pirateparty.gr%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce&tr=udp%3A%2F%2Fretracker.lanta-net.ru%3A2710%2Fannounce&tr=udp%3A%2F%2Fopen.demonii.si%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fipv4.tracker.harry.lu%3A80%2Fannounce&tr=udp%3A%2F%2F62.138.0.158%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2F62.138.0.158%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.zer0day.to%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969%2Fannounce&tr=udp%3A%2F%2Fcoppersurfer.tk%3A6969%2Fannounce"

Not working with $HOME variable.
Solution: use home;
//https://stackoverflow.com/questions/64918731/how-do-i-find-the-path-to-the-home-directory-for-linux
//https://docs.rs/home/0.5.3/home/

Creates files in ~ instead of ~/.mandmore
* Solution: https://doc.rust-lang.org/std/process/struct.Command.html
Create .mandmore
* 
* 
*/
