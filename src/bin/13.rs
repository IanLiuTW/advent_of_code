use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;

    let arrays = get_arrays(input);
    for array in arrays {
        ans += find_mirror_val(array);
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn get_arrays(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut arrays = vec![];

    let mut tmp = vec![];
    for line in input.lines() {
        if line.is_empty() {
            arrays.push(tmp.clone());
            tmp.clear();
        } else {
            tmp.push(line.chars().collect_vec());
        }
    }
    arrays.push(tmp.clone());
    arrays
}

fn find_mirror_val(mut array: Vec<Vec<char>>) -> u32 {
    if let Some(i) = find_mirror_idx(&array) {
        ((i + 1) * 100) as u32
    } else {
        array = transpose(array);
        let i = find_mirror_idx(&array).unwrap();
        (i + 1) as u32
    }
}

fn find_mirror_idx(array: &Vec<Vec<char>>) -> Option<usize> {
    (0..array.len() - 1).find(|&i| array[i] == array[i + 1] && is_mirrored(array, i))
}

fn is_mirrored(array: &Vec<Vec<char>>, i: usize) -> bool {
    let first = (0..=i).rev().map(|i| &array[i]);
    let second = (i + 1..array.len()).map(|i| &array[i]);
    for (l1, l2) in std::iter::zip(first, second) {
        if l1 != l2 {
            return false;
        }
    }
    true
}

fn transpose<T>(array: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..array[0].len())
        .map(|i| array.iter().map(|row| row[i].clone()).collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_transpose() {
        let array = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(transpose(array), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }
}
