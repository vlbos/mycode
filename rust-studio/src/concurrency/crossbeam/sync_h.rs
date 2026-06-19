
use crossbeam_channel::{Receiver, RecvError, Select};
pub fn recv_multiple<T>(rs: &[Receiver<T>]) -> Result<T, RecvError> {
    let mut sel = Select::new();
    for r in rs {
        sel.recv(r);
    }
    let oper = sel.select();
    let index = oper.index();
    oper.recv(&rs[index])
}

pub fn main() {}
