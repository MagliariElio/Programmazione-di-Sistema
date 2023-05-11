
#[derive(Debug)]
struct MyCycle<I: Clone + Iterator> {
    val: I,
    val_clone: I,
    repeat: usize
}

impl <I: Clone + Iterator> MyCycle<I> {
    fn new(iter: I, repeat_val: usize) -> Self {
        MyCycle {
            val_clone: iter.clone(),
            val: iter,
            repeat: repeat_val
        }
    }
}

impl <I: Clone + Iterator> Iterator for MyCycle<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.val.next() {
            Some(x) => Some(x),
            None => {
                self.repeat -= 1;
                if self.repeat <= 0 {
                    None
                } else {
                    self.val = self.val_clone.clone();
                    self.val.next()
                }
            }
        }
    }
}

fn main() {
    let iter = vec![0, 1, 2, 3];
    let mut repeat = 3;
    let mut my_cycle = MyCycle::new(iter.iter(), repeat.clone());

    while repeat != 0 {
        for i in 0..iter.len() {
            assert_eq!(my_cycle.next(), iter.get(i));
        }
        repeat -= 1;
    }

}
