#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use structopt::StructOpt;

#[derive(StructOpt)]
struct RunOptions {
    #[structopt(long)]
    to: String,
}

pub(crate) fn hello(to: &str) -> String {
    format!("Hello, {}!", to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_to_is_as_expected() {
        let to = "Kresna";
        let expected_result = format!("Hello, {}!", to);

        assert_eq!(hello(to), expected_result);
    }
}

fn main() {
    let opt = RunOptions::from_args();
    println!("{}", hello(&opt.to));
}
