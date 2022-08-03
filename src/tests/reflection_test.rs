use reflection::{CanBe, CastChecker};
use reflection_macro_attribute::polymorphic;

#[polymorphic]
trait Renderable {}
trait Collider {}

struct A;
struct B;

impl Renderable for A {}
impl Collider for B {}


#[test]
fn tests() {
    assert!(CastChecker::<A, dyn Renderable>::new().check());
    assert!(!CastChecker::<B, dyn Renderable>::new().check());
}
