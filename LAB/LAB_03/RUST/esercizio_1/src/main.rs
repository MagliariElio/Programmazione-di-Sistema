
#[derive(Debug)]
struct T {
    val: i32
}

impl T {
    pub fn new(value: i32) -> Self {
        T {
            val: value
        }
    }

    pub fn modify(&mut self, value: i32) {
        self.val = value;
    }
}

#[derive(Debug)]
struct CircularBuffer {
    tape: Vec<T>,
    ind_r: usize,
    ind_w: usize,
    len: usize,
    max_len: usize
}

impl CircularBuffer {
    fn new(mut max: usize) -> Self {
        if max <= 0 {
            max = 1;
        }

        let mut buffer = CircularBuffer {
            tape: Vec::with_capacity(max),
            ind_r : 0,
            ind_w : 0,
            len: 0,
            max_len : max,
        };

        for _ in 0..buffer.max_len {
            buffer.tape.push(T::new(-1));
        }

        return buffer;
    }

    fn insert(&mut self, value: i32) {
        self.tape[self.ind_w] = T::new(value);
        self.ind_w = (self.ind_w + 1) % self.max_len;
        self.len += 1;
    }

    fn remove(&mut self) {
        if self.len <= 0 {
            println!("There are no elements in the vector");
            return;
        }

        self.tape[self.ind_r].modify(-1);
        self.ind_r = (self.ind_r + 1) % self.max_len;
        self.len -= 1;
    }
}


fn main() {
    let mut buffer = CircularBuffer::new(0);
    buffer.insert(1);
    buffer.insert(2);
    buffer.remove();
    buffer.insert(3);
    buffer.insert(4);
    buffer.insert(5);
    buffer.insert(6);
    buffer.insert(7);
    buffer.insert(8);

    println!("{:?}", buffer);


}
