#[macro_use]
extern crate hpm_derive;

trait HPM {
    fn hpm();
}

#[derive(HPM)]
struct FrenchToast;

#[derive(HPM)]
#[via(A)]
struct Waffles;

fn main() {
    FrenchToast::hpm();
    Waffles::hpm();
}
