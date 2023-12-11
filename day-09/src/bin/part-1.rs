fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| parse_line(line))
        .map(|values| derive_difference_vectors(values))
        .map(|values_with_differences| predict_next_value(values_with_differences))
        .sum()
}

fn parse_line(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|s| s.parse().expect("Line should include numbers"))
        .collect::<Vec<i64>>()
}

fn derive_difference_vector(values: Vec<i64>) -> Option<Vec<i64>> {
    // If there are less than 2 values, we can't derive a difference vector
    if values.len() < 2 || values.iter().all(|&x| x == 0) {
        return None;
    }

    let mut differences = Vec::new();

    for window in values.windows(2) {
        let difference = window[1] - window[0];
        differences.push(difference);
    }

    return Some(differences);
}

fn derive_difference_vectors(values: Vec<i64>) -> Vec<Vec<i64>> {
    let mut output = Vec::new();
    output.push(values);

    loop {
        let last = output.last().expect("Should be a latest output").clone();
        let difference_vector = derive_difference_vector(last);

        match difference_vector {
            Some(difference_vector) => output.push(difference_vector),
            None => break,
        }
    }

    output
}

fn predict_next_value(values_with_differences: Vec<Vec<i64>>) -> i64 {
    values_with_differences
        .iter()
        .rev()
        .map(|v| *v.last().expect("Should be a last value"))
        .reduce(|a, b| a + b)
        .expect("Predict next value should reduce to a i64")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_difference_vector_works() {
        let input = vec![0, 3, 6, 9, 12, 15];
        let result = derive_difference_vector(input);
        assert_eq!(result, Some(vec![3, 3, 3, 3, 3]));
    }

    #[test]
    fn derive_difference_vectors_works() {
        // 0   3   6   9  12  15
        //   3   3   3   3   3
        //     0   0   0   0
        let input = vec![0, 3, 6, 9, 12, 15];
        let result = derive_difference_vectors(input);
        assert_eq!(
            result,
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0],
            ]
        );

        //1   3   6  10  15  21
        //   2   3   4   5   6
        //     1   1   1   1
        //       0   0   0
        let input = vec![1, 3, 6, 10, 15, 21];
        let result = derive_difference_vectors(input);
        assert_eq!(
            result,
            vec![
                vec![1, 3, 6, 10, 15, 21],
                vec![2, 3, 4, 5, 6],
                vec![1, 1, 1, 1],
                vec![0, 0, 0],
            ]
        );
    }

    #[test]
    fn predict_next_value_works() {
        // 1   3   6  10  15  21  (28)
        //   2   3   4   5   6  (7)
        //   1   1   1   1   (1)
        //     0   0   0   (0)
        let input = vec![
            vec![1, 3, 6, 10, 15, 21],
            vec![2, 3, 4, 5, 6],
            vec![1, 1, 1, 1],
            vec![0, 0, 0],
        ];
        let result = predict_next_value(input);
        assert_eq!(result, 28);
    }

    #[test]
    fn parse_line_works() {
        let input = "0 3 6 9 12 15";
        let result = parse_line(input);
        assert_eq!(result, vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn it_works() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = part1(input);
        assert_eq!(result, 114);
    }
}
