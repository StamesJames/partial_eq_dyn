# dyn_partial_eq_derive

Use the derive DynPartialEq to derive the DynPartialEq trait which will be implementet by trying zu cast the other object of the comparisson to the implementor. Use PartialEqDyn to impement PartialEq with a componentwise comparisson that uses the eq method on all types that implement PartialEq and dyn_eq on all others.
