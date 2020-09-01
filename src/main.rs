use itertools::unfold;
use num_rational::Ratio;
use take_until::TakeUntilExt;
mod diagram;
mod svg;
use diagram::Diagram;
use structopt::StructOpt;

type Fraction = Ratio<u16>;
type Gram = u16;

#[derive(Debug, StructOpt)]
struct Args {
    /// How much food is in a can, in grams?
    #[structopt(short, long, default_value = "400")]
    can_contents: Gram,

    /// How much does an empty can weigh, in grams?
    #[structopt(short, long, default_value = "52")]
    empty_can: Gram,

    /// Filename for SVG output
    #[structopt(short, long)]
    svg: Option<String>,

    numerator: u16,
    denominator: u16,
}

fn main() {
    let args = Args::from_args();
    assert!(args.denominator > 0); // avoid divide by zero
    assert!(args.numerator > 0); // must take a reducing step
    assert!(args.numerator <= args.denominator); // must be a fraction of the whole

    let reduction = Fraction::new(args.numerator, args.denominator);

    let day_fractions = dogfood(reduction);

    let labeller = weight_labeller(args.can_contents, args.empty_can);
    let weight_labels: Vec<String> = day_fractions.iter().map(labeller).collect();

    println!("Starting with a full can...");
    for ((i, day), label) in day_fractions.iter().enumerate().zip(weight_labels.iter()) {
        println!("End of day {}: {} left over, {}", 1 + i, day, label);
    }

    if let Some(filename) = args.svg {
        // Convert the fractions into a series of stacked cans, each labelled with the end-of-day weight.
        let row = |weight| -> Diagram { Diagram::can().above(Diagram::label(weight)) };

        let diagram = weight_labels
            .into_iter()
            .fold(Diagram::new(), |diagram, weight| diagram.above(row(weight)));

        // Convert the diagram into an SVG file:
        svg::save(&diagram, &filename)
    }
}

fn dogfood(reduction: Fraction) -> Vec<Fraction> {
    let full_can = Fraction::new(1, 1);
    let is_empty = |can: &Fraction| can.numer() == &0;

    let step = |start: &Fraction| -> Fraction {
        if *start >= reduction {
            start - reduction
        } else {
            full_can + start - reduction
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_two_thirds() {
        assert_eq!(
            dogfood(Fraction::new(2, 3)),
            vec![
                Fraction::new(1, 3),
                Fraction::new(2, 3),
                Fraction::new(0, 3)
            ]
        );
    }
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
