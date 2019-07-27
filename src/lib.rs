fn helper<'a, 'b, T>(_: &'a &'b (), v: &'b T) -> &'a T { v }

/// Use black magic fuckery to turn any `&T` into a `&'static T`.
/// May introduce undefined behavior.
pub fn make_static<'a, T>(input: &'a T) -> &'static T {
    let f: fn(_, &'a T) -> &'static T = helper;
    f(&&(), input)
}
