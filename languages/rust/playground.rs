struct Number {
    odd: bool,
    value: i32
}

trait Signed {
    fn is_negative(self) -> bool;
}

impl Signed for Number {
    fn is_negative(self) -> bool {
        self.value < 0
    }
}



fn main() {
    let odd = Number { odd: false, value: -40 };
    let even = Number { odd: true, value: 0 };
    println!("{}", odd.is_negative());
    println!("{}", even.is_negative());
}
