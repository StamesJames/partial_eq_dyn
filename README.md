# Disclaimer

This is the first crate I published so I am new to making things production ready. Therefore use this crate with caution and feedback is welcome.

# partial_eq_dyn

Implement the DynPartialEq trait with explicit casting to enable comparison between any dyn objects. There also is an associated derive crate partial_eq_dyn_derive that derives the component wise implementation of PartialEq and uses dyn_eq for all dyn_objects.
Simply set AsAny and DynPartialEq as supertraits of all traits you want to use and then derive PartialEqDyn on all types you want to compare and that contain those traits. AsAny and DynPartialEq also can be derived automatically.

AsAny for example can be implemented like this. 
```
use std::any::Any;
use partial_eq_dyn::AsAny;
struct Test;
impl AsAny for Test{
    fn as_any(&self) -> &dyn Any {
        self
    }
}
let test_any: &dyn Any = Test.as_any();
```

And DynPartialEq can be implemented like this 

```
use std::any::Any;
use partial_eq_dyn::{DynPartialEq, AsAny};

trait TestTrait: DynPartialEq + AsAny{}

#[derive(PartialEq)]
struct Test;
impl TestTrait for Test{}
impl AsAny for Test{
   fn as_any(&self) -> &dyn Any {
       self
   }
}
impl DynPartialEq for Test{
    fn dyn_eq(&self, other: &dyn Any) -> bool {
        other
            .downcast_ref::<Test>()
            .map_or(false, |other| self == other)
    }
}
```