use std::{
    cmp::{Ordering, PartialOrd},
    convert::Into,
};
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

trait RectangleHelper: Clone + Into<f64> {}
impl<T: Clone + Into<f64>> RectangleHelper for T {}

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U>
where
    T: RectangleHelper,
    U: RectangleHelper,
{
    fn area(&self) -> f64 {
        self.width.clone().into() * self.height.clone().into()
    }
}

impl<T: RectangleHelper, U: RectangleHelper> PartialOrd for Rectangle<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

impl<T: RectangleHelper, U: RectangleHelper> PartialEq for Rectangle<T, U> {
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
    println!(
        "The largest rectangle is {:?}; area: {}",
        result,
        result.area(),
    );
}
