fn main() {
    println!("Add: {}", add(1.2, 2));
    println!("Subtract: {}", subtract(2, 1.1));
    println!("Mult: {}", mult(2, 1.1));
    println!("Div: {}", div(1, 0.25));
    println!("Module: {}", module(3, 2));
}

fn add<T, U>(x: T, y: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    x.into() + y.into()
}

fn subtract<T, U>(x: T, y: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    x.into() - y.into()
}

fn mult<T, U>(x: T, y: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    x.into() * y.into()
}

fn div<T, U>(x: T, y: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    x.into() / y.into()
}

fn module<T, U>(x: T, y: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    x.into() % y.into()
}

#[cfg(test)]
mod tests {
    use crate::{add, div, module, mult, subtract};

    #[test]
    fn test_add() {
        let x = 1.2;
        let y = 2;

        assert_eq!(add(x, y), f64::from(x) + f64::from(y));
    }

    #[test]
    fn test_subtract() {
        let x = 1.1;
        let y = 2;
        assert_eq!(subtract(x, y), f64::from(x) - f64::from(y));
    }

    #[test]
    fn test_mult() {
        let x = 2;
        let y = 4;
        assert_eq!(mult(x, y), f64::from(x) * f64::from(y));
    }

    #[test]
    fn test_div() {
        let x = 4;
        let y = 0.25;
        assert_eq!(div(x, y), f64::from(x) / f64::from(y));
    }

    #[test]
    fn test_module() {
        let x = 3;
        let y = 2;

        assert_eq!(module(x, y), f64::from(x) % f64::from(y));
    }
}
