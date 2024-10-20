fn arrays(){
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");
    println!("a: {a:#?}");
}

fn tuples() {
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
}

fn array_iteration() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}

fn patterns_and_destructuring() {
    fn print_tuple_1(tuple: (i32, i32)) {
        let left = tuple.0;
        let right = tuple.1;
        println!("left: {left}, right: {right}");
    }
    fn print_tuple_2(tuple: (i32, i32)) {
        let (left, right) = tuple;
        println!("left: {left}, right: {right}");
    }
    let tuple = (1,2);
    print_tuple_1(tuple);
    print_tuple_2(tuple);
}


//
// Exercise: Nested Arrays
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3 ]; 3];
    for (i, mat) in matrix.iter().enumerate() {
        for (j, m) in mat.iter().enumerate() {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn nested_arrays() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}


fn main() {
    nested_arrays();


    //

    // patterns_and_destructuring();
    // array_iteration();
    // tuples();
    // arrays();
}
