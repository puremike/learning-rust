pub fn add_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

pub fn mean_numbers(numbers: &[i32]) -> f64 {
    let sum: f64 = add_numbers(numbers) as f64;
    sum / numbers.len() as f64
}

// Add Numbers and Mean Number Functions with Generics

pub fn add_numbers_generics<T>(numbers: &[T]) -> f64
where
    T: Copy + Into<f64>,
{
    numbers.iter().map(|&n| n.into()).sum()
}

pub fn mean_numbers_generics<T>(numbers: &[T]) -> f64
where
    T: Copy + Into<f64>,
{
    let sum: f64 = add_numbers_generics(numbers);
    sum / numbers.len() as f64
}

pub fn calculate() {
    let x: Vec<i32> = vec![1, 2, 3, 4, 5];
    let m: Vec<f64> = vec![1.3, 2.0, 3.7, 14.0, 5.8];
    let y: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum: i32 = add_numbers(&x);
    let sum1: f64 = add_numbers_generics(&x);
    let sum2: f64 = add_numbers_generics(&m);

    println!("The Sum of {:?} is {}", &x, sum);
    println!("The Sum of {:?} using generics is {}", &x, sum1);
    println!("The Sum of {:?} using generics is {}", &m, sum2);

    let mean: f64 = mean_numbers(&y);
    let mean2: f64 = mean_numbers_generics(&y);
    println!("The Mean of {:?} is {}", &y, mean);
    println!("The Mean of {:?} using generics is {}", &y, mean2);
}
