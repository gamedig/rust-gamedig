use clap::Parser;

#[derive(Parser)]
#[command(name = "id_gen", version, about = "Generate game IDs for GameDig")]
struct Args {
    /// Game title exactly as on storefront
    #[arg(short, long)]
    title: String,

    /// Mod name (optional; appended if present)
    #[arg(short, long = "mod")]
    mod_name: Option<String>,

    /// Edition (optional; appended if present)
    #[arg(short, long)]
    edition: Option<String>,

    /// Release year (optional; appended if present)
    #[arg(short, long)]
    year: Option<u16>,
}

impl Args {
    fn normalize(input: &str, cap: usize) -> String {
        input
            .trim()
            .chars()
            .fold(
                (String::with_capacity(cap), false),
                |(mut acc, prev_us), ch| {
                    if ch.is_ascii_alphanumeric() {
                        acc.push(ch.to_ascii_lowercase());

                        (acc, false)
                    } else if !prev_us {
                        acc.push('_');

                        (acc, true)
                    } else {
                        (acc, true)
                    }
                },
            )
            .0
            .trim_matches('_')
            .to_string()
    }

    /// Generate the identifier: <title>[_<mod>][_<edition>][_<year>]
    fn generate_id(&self) -> String {
        let mut id = Self::normalize(&self.title, 64);

        // Mod segment (if present)
        if self.mod_name.is_some() {
            id.push('_');
            id.push_str(&Self::normalize(self.mod_name.as_ref().unwrap(), 16));
        }

        // Edition segment (if present)
        if self.edition.is_some() {
            id.push('_');
            id.push_str(&Self::normalize(self.edition.as_ref().unwrap(), 16));
        }

        // Year segment (if present)
        if self.year.is_some() {
            id.push('_');
            id.push_str(&self.year.unwrap().to_string());
        }

        id
    }
}

fn main() {
    println!("{}", Args::parse().generate_id());
}
