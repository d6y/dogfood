use itertools::unfold;
use num_rational::Ratio;
use take_until::TakeUntilExt;
mod svg;

type Fraction = Ratio<u16>;

fn main() {
    let numer = 1;
    let denom = 5;

    assert!(denom > 0); // avoid divide by zero
    assert!(numer > 0); // must take a reducing step
    assert!(numer <= denom); // must be a fraction of the whole

    let reduction = Fraction::new(numer, denom);

    let day_fractions = dogfood(reduction);

    for day in &day_fractions {
        println!("{}", day);
    }

    // let can_contents_weight_g = 400;
    // let empty_can_weight_g = 52;

    // let weight = |remaining: &Rational| -> String {
    //     if *remaining == *can::FULL {
    //         String::from("Full")
    //     } else if *remaining == *can::EMPTY {
    //         String::from("Empty")
    //     } else {
    //         format!(
    //             "{:.0}g",
    //             empty_can_weight_g as f32
    //                 + can_contents_weight_g as f32
    //                     * (*remaining.numer() as f32 / *remaining.denom() as f32)
    //         )
    //     }
    // };

    // let labels: Vec<String> = day_fractions.iter().map(weight).collect();

    // println!("Starting from a full can:");
    // for day in &day_fractions {
    //     println!("{:?} {}", day, weight(day));
    // }

    // svg::draw("image.svg", &day_fractions, &labels);
}

fn dogfood(reduction: Fraction) -> Vec<Fraction> {

    let full_can = Fraction::new(1, 1);
    let empty_can = Fraction::new(0, 1);

    let is_empty = |can: &Fraction| can.numer() == &0;

    let step = |start: &Fraction| -> Fraction {
        let remains = start - reduction;
        if remains >= empty_can {
            remains
        } else {
            full_can + remains
        }
    };

    unfold(full_can, move |start| {
        let end = step(start);
        *start = end.clone();
        Some(end)
    })
    .take_until(is_empty)
    .take(100) // safety net on how long this runs for
    .collect()
}
