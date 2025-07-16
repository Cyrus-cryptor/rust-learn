fn main() {
    // let width = 30;
    // let heigh = 40;
    // println!("The area is {}", area(width, heigh));

    // let rect1 = (30, 40);
    // println!("The area is {}", area_by_tuple(rect1));

    let rectangle = Rectangle {
        width: 20,
        heigh: 60,
    };
    println!("rectangle is {rectangle:#?}");
    dbg!(&rectangle);
    println!("{}", rectangle.heigh);
    println!(
        "The area is {} for width: {} and heigh: {}",
        area_by_rectangle(&rectangle),
        rectangle.width,
        rectangle.heigh
    );
}

fn area(width: u32, heigh: u32) -> u32 {
    width * heigh
}

fn area_by_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_by_rectangle(rect: &Rectangle) -> u32 {
    rect.heigh * rect.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigh: u32,
}
