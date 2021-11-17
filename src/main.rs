fn main() {
    let mut val0 = "val0";
    println!("{}", val0);
    val0 = "val0-1";
    println!("{}", val0);

    let val91: Option<String> = Some("sample".to_string());
    println!("{:?}", val91);
    println!("{}", val91.unwrap());

    let mut val1 = "val1";
    println!("var val1: {}", val1);
    {
        println!("var val1: {}", val1);

        let val2 = &mut val1;
        println!("var val2: {}", val2);

        *val2 = &"val2";
        println!("var val2: {}", val2);
    }

    println!("var val1: {}", val1);
    val1 = &"val1.1";
    println!("var val1: {}", val1);

    let mut nums: Vec<f64> = Vec::new();
    nums.push(1.1);
    nums.push(2.3);

    println!(
        "{}",
        calc(
            nums.pop().expect("error?"),
            nums.pop().expect("error??"),
            |x, y| x + y
        )
    );
}

fn calc<F>(x: f64, y: f64, calc: F) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    calc(x, y)
}
