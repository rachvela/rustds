
fn main() {

    let mut list = ds::list::LinkedList::new(121);
    list.insert(124, 0);
    list.insert(122, 0);
    list.insert(999, 10);
    list.insert(123, 1);

    list.print();
    list.insert(1, 0);

    print!("Find {:?}\n", list.find(123));

    list.print();
    print! ("removed {:?}\n", list.remove(0));
    list.print();
    print! ("removed {:?}\n", list.remove(3));
    list.print();
    print! ("removed {:?}\n", list.remove(10));
    list.print();

//    let to_delete = list.len();
    for _ in 0..list.len() {
        print! ("removed {:?}\n", list.remove(0));
    }
    list.print();
}
