use std::any::Any;

/// Use this to implement the functionality to cast any type zu a &dyn Any.
/// Implement simply with:
/// ```
/// # use std::any::Any;
/// # use dyn_partial_eq::AsAny;
/// struct Test;
/// impl AsAny for Test{
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
/// }
/// let test_any: &dyn Any = Test.as_any();
/// ```
/// or use the derive macro
/// types you whant to make dynamicaly comparable need this as supertrait
pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
}

/// Use this to implement the functionality to compare a Type to an Any Object.
/// Most of the times you want your own type to also be an AsAny implementor so the implementation of DynPartialEq becomes a simple downcast like this:
/// ```
/// # use std::any::Any;
/// # use dyn_partial_eq::{DynPartialEq, AsAny};
///
/// trait TestTrait: DynPartialEq + AsAny{}
/// #[derive(PartialEq)]
/// struct Test;
/// impl TestTrait for Test{}
/// # impl AsAny for Test{
/// #    fn as_any(&self) -> &dyn Any {
/// #        self
/// #    }
/// # }
/// impl DynPartialEq for Test{
///     fn dyn_eq(&self, other: &dyn Any) -> bool {
///         other
///             .downcast_ref::<Test>()
///             .map_or(false, |other| self == other)
///     }
/// }
/// let test_dyn1: &dyn TestTrait = &Test;
/// let test_dyn2: &dyn TestTrait = &Test;
/// assert!(test_dyn1.dyn_eq(test_dyn2.as_any()));
/// ```
pub trait DynPartialEq {
    fn dyn_eq(&self, other: &dyn Any) -> bool;
}
