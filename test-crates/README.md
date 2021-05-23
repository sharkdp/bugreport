This directory contains crates used to test `bugreport`.

It is insufficient to test bugreport with regular crate test facilities, due to
how bugreport is implemented.

Since bugreport uses a `bugreport!()` macro that is evaluated in the context of
a dependent crate, it is important to test evaluation of that macro in an
external create. Otherwise the test will not detect problems such as having
`#[cfg(feature = "git_hash")]` inside the `bugreport!()` macro. Since the macro
is evaluated in the context of the dependent crate, that feature will not be
defined in that context.
