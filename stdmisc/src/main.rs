
use std::thread;



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


fn main() {
    RunThread();
}
