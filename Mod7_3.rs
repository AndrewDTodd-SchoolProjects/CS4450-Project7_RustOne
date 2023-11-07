fn f2n<T>(f: fn(T) -> T, n: u32, x: T) -> T
{
    let mut result = x;
    for _ in 0..n
    {
        result = f(result);
    }
    result
}

fn main()
{
    fn sq(x: f64) -> f64
    {
        x * x
    }
    let x: f64 = 2.0;
    println!("{:?}", f2n(sq, 3, x.clone()));

    fn pluralize(word: String) -> String
    {
        word.clone() + "+"
    }
    let s = String::from("foo");
    println!("{:?}", f2n(pluralize, 3, s.clone()));
}
