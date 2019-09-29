pub fn wrap<T>(v: T, min: T, max: T) -> T
where
    T: std::cmp::PartialOrd,
{
    if v > max {
        min
    } else if v < min {
        max
    } else {
        v
    }
}
