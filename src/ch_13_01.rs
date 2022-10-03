use std::{thread, time};

#[derive(Debug)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preferece: Option<ShirtColor>) -> ShirtColor {
        user_preferece.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;
        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            }
        }
        if red_count > blue_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn ch_13_01() {
    let inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };
    let user_preferece = Some(ShirtColor::Blue);
    let giveaway1 = inventory.giveaway(user_preferece);
    dbg!(giveaway1);

    let giveaway2 = inventory.giveaway(None);
    dbg!(giveaway2);

    let mut list = vec![1, 2, 3];
    dbg!(&list);

    let mut borrow_mutably = || list.push(5);
    borrow_mutably();
    dbg!(&list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    fn some_long_func() {
        let duration = time::Duration::from_millis(10);
        let start = time::Instant::now();
        thread::sleep(duration);
        let end = time::Instant::now();
        dbg!(end.duration_since(start));
    }

    thread::spawn(some_long_func).join().unwrap();
}
