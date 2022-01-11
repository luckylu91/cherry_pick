use std::iter;

pub fn pairs_of<'a, T>(v: Vec<T>) -> Option<Box<dyn Iterator<Item = (T, T)> + 'a>>
where T: Copy + 'a
{
    let n = v.len();
    if n == 0 {
        return None;
    }
    let it = (0..=(n - 1))
        .map(move |k| iter::repeat(k).zip(k..=(n - 1)))
        .flat_map(|r| r)
        .map(move |(k1, k2)| (v[k1], v[k2]));
    Some(Box::new(it))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn void() {
        if let None = pairs_of(vec![] as Vec<i32>) {
        } else {
            panic!("")
        }
    }

    #[test]
    fn one() {
        if let Some(it) = pairs_of(vec![0] as Vec<i32>) {
            let v = it.collect::<Vec<(i32, i32)>>();
            println!("{:?}", &v);
            assert_eq!(
                v,
                vec![(0, 0)]
            )
        } else {
            panic!("")
        }
    }

    #[test]
    fn two() {
        if let Some(it) = pairs_of(vec![0, 1] as Vec<i32>) {
            let v = it.collect::<Vec<(i32, i32)>>();
            println!("{:?}", &v);
            assert_eq!(
                v,
                vec![(0, 0), (0, 1), (1, 1)]
            )
        } else {
            panic!("")
        }
    }

    #[test]
    fn three() {
        if let Some(it) = pairs_of(vec![0, 1, 2] as Vec<i32>) {
            let v = it.collect::<Vec<(i32, i32)>>();
            println!("{:?}", &v);
            assert_eq!(
                v,
                vec![(0, 0), (0, 1), (0, 2), (1, 1), (1, 2), (2, 2)]
            )
        } else {
            panic!("")
        }
    }
}
