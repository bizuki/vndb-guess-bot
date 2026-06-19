/// A VNDB field or sort selector.
///
/// This is intentionally narrower than `Display`: request construction only
/// needs values that know how to produce a VNDB selector string.
pub trait VndbSelector {
    fn selector(&self) -> String;
}

impl<T> VndbSelector for &T
where
    T: VndbSelector + ?Sized,
{
    fn selector(&self) -> String {
        (*self).selector()
    }
}

impl<T> VndbSelector for Box<T>
where
    T: VndbSelector + ?Sized,
{
    fn selector(&self) -> String {
        (**self).selector()
    }
}
