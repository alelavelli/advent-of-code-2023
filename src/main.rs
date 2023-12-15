use advent_of_code_2023::day_14;
use ndarray::{arr2, Axis};

fn main() {

    /*let mut a = arr2(&vec![
        [1,2,3],
        [4,5,6]
    ]);
    println!("a: \n{:?}", a);
    a = a.reversed_axes();
    println!("a: \n{:?}", a);
    a.invert_axis(Axis(1));
    a = a.reversed_axes();
    println!("a: \n{:?}", a);
    a.invert_axis(Axis(1));
    a = a.reversed_axes();
    a.invert_axis(Axis(1));
    println!("a: \n{:?}", a);
    a.invert_axis(Axis(1));
    a = a.reversed_axes();
    a.invert_axis(Axis(1));
    a.invert_axis(Axis(0));
    println!("a: \n{:?}", a);*/
    day_14::solve_pt2();
}