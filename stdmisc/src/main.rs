
use std::thread;

use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, BufRead};

static count:i32=5;

fn RunChannels(){
    let (tx,rx): (Sender<i32>,Receiver<i32>)=mpsc::channel();

    let mut cha_list = Vec::new();

    // TX
    for id in 0..5 {
        let thr_tx = tx.clone();

        let cha =thread::spawn(move||{
            thr_tx.send(id).unwrap();
            println!(" TX {}",id);
        });
        cha_list.push(cha);
    }

    // RX
    let mut ids = Vec::with_capacity(count as usize);

    for _ in 0..count{
        ids.push(rx.recv());
    }
    
    // WAIT
    for cha in cha_list{
        cha.join().expect("bad panick");
    }
    println!("{:?}" ,ids);
}



fn RunThread(){
    let mut tlist = vec![];

    for i in 0..10 {

        tlist.push(thread::spawn(move||{
            println!("Thread num {}",i);
        }))
    }

    for t in tlist {
        let _ = t.join();
    }
}

fn MakeFile()
{
   // Create a path to the desired file
   let path = Path::new("hello.txt");
   let display = path.display();

   let mut file;
   
   file = match File::create(&path){
       Err(why) => panic!("create fail {}",why),
       Ok(file) => file,
   };

   match file.write_all("hello test1\nhello test2\nhello test3\n".as_bytes()){
       Err(why) => panic!("write fail {}",why),
       Ok(_)=>println!("written"),
   }

   file= match File::open(&path) {
       Err(why) => panic!("----------- {}: {}", display, why),
       Ok(file) => file,
   };

   
 
   let mut s = String::new();
   match file.read_to_string(&mut s) {
       Err(why) => panic!("-------------- {}: {}", display, why),
       Ok(_) => print!("{} contains:\n{}", display, s),
   }


    for line in io::BufReader::new(file).lines() {
        if let Ok(ip) = line {
            println!("line {}", ip);
        }
    
}

}

fn main() {
    RunThread();
    RunChannels();
    MakeFile();
}
