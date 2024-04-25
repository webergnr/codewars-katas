// https://www.codewars.com/kata/58ca658cc0d6401f2700045f/train/rust

// In this simple exercise, you will build a program that takes a value, integer , and returns a list of its multiples up to another value, limit . If limit is a multiple of integer, it should be included as well. There will only ever be positive integers passed into the function, not consisting of 0. The limit will always be higher than the base.
// For example, if the parameters passed are (2, 6), the function should return [2, 4, 6] as 2, 4, and 6 are the multiples of 2 up to 6.
fn main() {
    //
}

fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    let mut v = vec![];


    for i in n..limit+1 {
        println!("############");
        println!("VALUE {}", n);
        println!("FOR I {}", i);
        println!("REST {}", i.rem_euclid(n));

        if i.rem_euclid(n) == 0 {
            v.push(i)
        }
    }
    v
}


#[cfg(test)]
mod tests {
    use super::find_multiples;

    #[test]
    fn basic_test() {
        assert_eq!(find_multiples(4, 27), [4, 8, 12, 16, 20, 24]);
        assert_eq!(find_multiples(1, 2), [1, 2]);
        assert_eq!(find_multiples(5, 7), [5]);
        assert_eq!(find_multiples(11, 54), [11, 22, 33, 44]);
        assert_eq!(find_multiples(5, 25), [5, 10, 15, 20, 25]);
    }
}

