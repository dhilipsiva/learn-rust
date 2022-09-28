use std::{
    cmp::{Ordering, PartialOrd},
    convert::Into,
    ops::Mul,
};
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    &largest
}

#[derive(Debug)]
struct Rectangle<T: Mul<Output = T> + Clone + Into<f64>, U: Mul<Output = U> + Clone + Into<f64>> {
    width: T,
    height: U,
}

impl<T: Mul<Output = T> + Clone + Into<f64>, U: Mul<Output = U> + Clone + Into<f64>>
    Rectangle<T, U>
{
    fn area(&self) -> f64 {
        &self.width.clone().into() * &self.height.clone().into()
    }
}

impl<T: Mul<Output = T> + Clone + Into<f64>, U: Mul<Output = U> + Clone + Into<f64>> PartialOrd
    for Rectangle<T, U>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
}
impl<T: Mul<Output = T> + Clone + Into<f64>, U: Mul<Output = U> + Clone + Into<f64>> PartialEq
    for Rectangle<T, U>
{
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }
}

pub fn ch_10_02() {
    let num_list = vec![1, 10, 30, 12, 4];
    println!("The largest int is {}", largest(&num_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let rect_list = vec![
        Rectangle {
            width: 5,
            height: 10.5,
        },
        Rectangle {
            width: 7,
            height: 9.8,
        },
        Rectangle {
            height: 0.5,
            width: 10,
        },
    ];
    let result = largest(&rect_list);
    println!("The largest rectangle is {:?}", result);
}
