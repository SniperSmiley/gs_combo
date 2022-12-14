use std::time::Instant;
use golfscript_rs;
fn main() {
  let s = ['_','a', 'b', 'c', 'd','0','1','2','3','4','5','6','7','8','9','~','`','!','@','$','+','-','*','/','%','|','&','^','{','}','\'','"','[',']','\\',':',':','<','>','=',',','.','?','(',')','p','n'];
  let ss = s.len();
  let mut i = 0;
  
  loop {
      let mut blerb = "".to_string();
      let mut n = i;
      while n > 0 {
          blerb = s[n % ss].to_string() + blerb.as_str();
          n /= ss;
      }
      let start = Instant::now();
      let hump=golfscript_rs::golfscript("".to_string(), blerb.clone());
      let elapsed = start.elapsed();
      i += 1;
      let k = hump.as_bytes();
      if elapsed.as_secs()>5{println!("Too long")}
      if elapsed.as_secs()>5||(1000>k.len() && k.len()>3 && k[0]==48u8 && k[1]==10u8 && k[2]==51u8)
      {
      print!("{}\n{}",blerb,hump)}
  } 
}
