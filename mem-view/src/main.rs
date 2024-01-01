mod structs;
use structs::*;
use mem_dbg::*;

fn main(){
    
    let mut s = UserList::new();

    for _ in 0..0{
        let info = User::new();
        s.put(info);
    }

    println!("size:     {}", s.mem_size(SizeFlags::default()));
    println!("capacity: {}", s.mem_size(SizeFlags::CAPACITY));
    
    s.mem_dbg(DbgFlags::default()).unwrap();
    loop{}
}