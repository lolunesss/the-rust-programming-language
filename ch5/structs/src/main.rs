#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width: 10,
        height: 40,
    };
    let rect3 = Rect {
        width: 60,
        height: 45,
    };

    println!("Area of rectangle is {}", area(&rect));
    println!("Area of rectangle is {}", rect.area());
    println!("Rect : {rect:#?}");
    println!("Can rect hold  rect2 ? {}", rect.can_hold(&rect2));
    println!("Can rect hold  rect3 ? {}", rect.can_hold(&rect3));
    println!("Square of size 10: {:#?}", Rect::square(20));
    dbg!(&rect);
}
