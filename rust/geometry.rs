pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub fn summarize_rectangles(items: &[Rectangle]) -> (f64, f64, f64, usize) {
    let mut total_area = 0.0;
    let mut largest = 0.0;
    let mut smallest = f64::MAX;
    let mut count = 0;
    for item in items {
        let area = item.width * item.height;
        if area <= 0.0 {
            continue;
        }
        total_area += area;
        if area > largest {
            largest = area;
        }
        if area < smallest {
            smallest = area;
        }
        count += 1;
    }
    if count == 0 {
        smallest = 0.0;
    }
    (total_area, largest, smallest, count)
}
