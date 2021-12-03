pub fn tails<T>(data: &[T]) -> impl Iterator<Item = (&T, &[T])> + '_ {
    data.iter()
        .enumerate()
        .map(move |(idx, v)| (v, &data[idx + 1..]))
}
