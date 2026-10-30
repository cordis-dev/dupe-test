pub struct Invoice {
    pub quantity: f64,
    pub price: f64,
}

pub fn summarize_invoices(lines: &[Invoice]) -> (f64, f64, f64, usize) {
    let mut grand_total = 0.0;
    let mut biggest = 0.0;
    let mut cheapest = f64::MAX;
    let mut counted = 0;
    for line in lines {
        let amount = line.quantity * line.price;
        if amount <= 0.0 {
            continue;
        }
        grand_total += amount;
        if amount > biggest {
            biggest = amount;
        }
        if amount < cheapest {
            cheapest = amount;
        }
        counted += 1;
    }
    if counted == 0 {
        cheapest = 0.0;
    }
    (grand_total, biggest, cheapest, counted)
}
