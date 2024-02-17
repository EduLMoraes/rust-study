// Fribonnaci sem recursividade e sem armazenamento de vetor.

fn main() {
    assert_eq!(fribonacci(0), 0);
    assert_eq!(fribonacci(5), 5);
    assert_eq!(fribonacci(2), 1);
    assert_eq!(fribonacci(1), 1);
    assert_eq!(fribonacci(8), 21);
    assert_eq!(fribonacci(6), 8);
    assert_eq!(fribonacci(7), 13);
    assert_eq!(fribonacci(3), 2);
    assert_eq!(fribonacci(-13), 0);
    assert_eq!(fribonacci(100), 354224848179261915075);
}

fn fribonacci(val: i128) -> i128{
    if val <= 0{
        return 0;
    }else if val == 1 || val == 2{
        return 1;
    }

    let mut one: i128 = 0;
    let mut two: i128 = 1;
    let mut result: i128 = 0;

    for _ in 1..val{

        result = one + two;

        one = two;
        two = result;
    }

    return result;
}
