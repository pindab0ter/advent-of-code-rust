use std::iter::once;

pub fn rotations<T: Copy>(xs: &[T]) -> impl Iterator<Item = impl Iterator<Item = T>> {
    (0..xs.len()).map(|i| xs.iter().skip(i).chain(xs.iter().take(i)).copied())
}

pub fn permutations<T: Copy>(xs: &[T]) -> Box<dyn Iterator<Item = Vec<T>> + '_> {
    if xs.is_empty() {
        return Box::new(once(vec![]));
    }

    Box::new(rotations(xs).flat_map(|rotation| {
        let rotation = rotation.collect::<Vec<T>>();
        let (first, rest) = rotation.split_first().unwrap();
        permutations(rest)
            .map(|permutation| once(*first).chain(permutation).collect())
            .collect::<Vec<_>>()
    }))
}