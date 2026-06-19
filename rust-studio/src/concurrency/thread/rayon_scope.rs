pub fn rayon_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    rayon::scope(|s| {
        s.spawn(|_| {
            println!("hello from the first rayon scoped thread");
            dbg!(&a);
        });
        s.spawn(|_| {
            println!("hello from the second rayon scoped thread");
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });

    a.push(4);
    assert_eq!(x, a.len());
}
// After the scope, we can modify and access our variables again:
pub fn main() {
    rayon_scope();
    rayon::scope_fifo(|s| {
        s.spawn_fifo(|s| {
            // task s.1
            s.spawn_fifo(|_s| {
                // task s.1.1
                rayon::scope_fifo(|t| {
                    t.spawn_fifo(|_| ()); // task t.1
                    t.spawn_fifo(|_| ()); // task t.2
                });
            });
        });

        s.spawn_fifo(|_s| { // task s.2
        });
        // point mid
    }); // point end
}
