use itertools::unfold;
use num_rational::Rational;
use take_until::TakeUntilExt;
mod can;
mod svg;

fn main() {
    let denom = 5;
    assert!(denom > 0); // avoid divide by zero
    assert!(denom < 20); // nb: larger the denominator, the longer the sequence

    let numerator = 1;
    assert!(numerator >= 1); // must take a reducing step
    assert!(numerator <= denom); // must be a fraction of the whole

    let reduction = Rational::new(numerator, denom);
    let day_fractions = dogfood(reduction);

    let can_contents_weight_g = 400;
    let empty_can_weight_g = 52;

    let weight = |remaining: &Rational| -> String {
        if *remaining == *can::FULL {
            String::from("Full")
        } else if *remaining == *can::EMPTY {
            String::from("Empty")
        } else {
            format!(
                "{:.0}g",
                empty_can_weight_g as f32
                    + can_contents_weight_g as f32
                        * (*remaining.numer() as f32 / *remaining.denom() as f32)
            )
        }
    };

    let labels: Vec<String> = day_fractions.iter().map(weight).collect();

    println!("Starting from a full can:");
    for day in &day_fractions {
        println!("{:?} {}", day, weight(day));
    }

    svg::draw("image.svg", &day_fractions, &labels);
}

fn dogfood(reduction: Rational) -> Vec<Rational> {
    let step = |start: &Rational| -> Rational {
        let remains = start - reduction;
        if remains >= *can::EMPTY {
            remains
        } else {
            *can::FULL + remains
        }
    };

    unfold(*can::FULL, move |start| {
        let end = step(start);
        *start = end.clone();
        Some(end)
    })
    .take_until(can::is_empty)
    .take(100) // safety net on how long this runs for
    .collect()
}
