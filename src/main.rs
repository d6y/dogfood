use itertools::unfold;
use num_rational::Ratio;
use take_until::TakeUntilExt;
mod diagram;
mod svg;
use diagram::Diagram;

type Fraction = Ratio<u16>;
type Gram = u16;

fn main() {
    let numer = 1;
    let denom = 5;

    assert!(denom > 0); // avoid divide by zero
    assert!(numer > 0); // must take a reducing step
    assert!(numer <= denom); // must be a fraction of the whole

    let can_contents: Gram = 400; // how much food in a can?
    let empty_can: Gram = 52; // what does an empty can weigh?

    let reduction = Fraction::new(numer, denom);

    let day_fractions = dogfood(reduction);

    println!("Starting with a full can...");
    for day in &day_fractions {
        println!("{}", day);
    }

    let labeller = weight_labeller(can_contents, empty_can);
    let weight_labels = day_fractions.iter().map(labeller);

    let row = |weight| -> Diagram { Diagram::can().above(Diagram::label(weight)) };

    let diagram = &weight_labels.fold(Diagram::new(), |diagram, weight| diagram.above(row(weight)));

    dbg!(diagram);

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

fn weight_labeller(can_contents: Gram, empty_can: Gram) -> impl Fn(&Fraction) -> String {
    move |remaining| {
        if *remaining.numer() == 0 {
            String::from("Empty")
        } else {
            format!(
                "{:.0}g",
                empty_can as f32
                    + can_contents as f32 * (*remaining.numer() as f32 / *remaining.denom() as f32)
            )
        }
    }
}
