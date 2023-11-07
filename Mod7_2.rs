enum Shape
{
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape
{
    fn area(&self) -> f64
    {
        match *self
        {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(length, width) => length * width,
        }
    }
}

fn main()
{
    let c = Shape::Circle(3.0);
    println!("Circle area: {}", c.area()); //28.274333882308138
    let r = Shape::Rectangle(6.0, 7.0);
    println!("Rectangle area: {}", r.area()); //42
}
