use max_matching_rs::MatchingGraph;
mod utils;
use utils::utils::matching_permutations;

#[cfg(test)]
mod tests {

    use utils::utils::Permutations;

    use super::*;

    #[test]
    fn test0() {
        // staff_data.0 => name
        // staff_data.1 => capable
        let staff_data = vec![
            ("1".to_string(), vec!["B".to_string(), "D".to_string()]),
            (
                "2".to_string(),
                vec!["A".to_string(), "C".to_string(), "E".to_string()],
            ),
            ("3".to_string(), vec!["B".to_string()]),
            (
                "4".to_string(),
                vec!["D".to_string(), "E".to_string(), "F".to_string()],
            ),
            ("5".to_string(), vec!["B".to_string(), "D".to_string()]),
        ];
        let works_data = [
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
        ];
        let staff_nodes: Vec<usize> = staff_data.iter().enumerate().map(|(i, _)| i).collect();
        let works_nodes: Vec<usize> = works_data.iter().enumerate().map(|(i, _)| i).collect();
        let mut mgraph = MatchingGraph::new(staff_nodes, works_nodes);

        for (i, j) in staff_data.iter().enumerate() {
            for k in j.1.clone() {
                let indexof = works_data.iter().position(|l| l == &k).unwrap();
                mgraph.add_side(i, indexof);
            }
        }

        println!("最大マッチング {:?}", mgraph.max_matching());
        for (a, b) in mgraph.max_matching() {
            println!("{} - {}", staff_data[a].0, works_data[b])
        }
        println!("{:?}", mgraph.max_matching2())
    }

    #[test]
    fn test1() {
        let arr = vec![0, 1, 2, 3];
        for i in matching_permutations(arr, 0) {
            //pass
            println!("{:?}", i);
        }
    }

    #[test]
    fn test2() {
        //変換のテスト
        for i in Permutations::new(vec![1, 2, 3, 4]) {
            println!("{:?}", i);
        }
    }
}
