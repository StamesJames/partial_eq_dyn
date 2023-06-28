# dyn_partial_eq_derive

To implement PartialEq on Types with TraitObject fields you can use the derive Macro PartialEqDyn. The Implementation needs the Traits that are present as TraitObjects to have AsAny and DynPartialEq as supertraits. For those Traits there also exist derive macros AsAny and DynPartialEq.
