#[derive(Debug, Clone)]
struct Node {
    pub tag: char,
    pub dep_on: String,
    pub in_degree: usize,
}

impl Node {
    fn new(tag: char, dep_on: impl Into<String>) -> Node {
        Node {
            tag,
            dep_on: dep_on.into(),
            in_degree: 0,
        }
    }
}

fn init_degree(nodes: &mut Vec<Node>) {
    // init in_degree
    for i in 0..nodes.len() {
        let deps = nodes[i].dep_on.clone();
        for t in deps.chars() {
            match nodes.iter_mut().find(|n| n.tag == t) {
                Some(n) => n.in_degree += 1,
                None => {}
            }
        }
    }
    dbg!(&nodes);
}

///
/// O(n^2)
///
fn t_sort1(mut nodes: Vec<Node>) -> String {
    let mut result = String::new();
    init_degree(&mut nodes);
    let mut depth = 0;
    while nodes.len() > 0 {
        if depth > 100_000 {
            panic!("looooop");
        }

        // search in-degree zero
        if let Some((i, z_node)) = nodes.iter().enumerate().find(|(_i, n)| n.in_degree == 0) {
            result.push(z_node.tag);
            // decrease in-degree of zero's deps
            let dep = z_node.dep_on.to_string();
            for t in dep.chars() {
                match nodes.iter_mut().find(|n| n.tag == t) {
                    Some(n) => n.in_degree -= 1,
                    None => {}
                }
            }
            nodes.remove(i);
        }
        depth += 1;
    }
    result
}

fn t_sort2(mut nodes: Vec<Node>) -> String {
    init_degree(&mut nodes);
    let mut res = String::new();
    let mut zeros: Vec<Node> = nodes.iter().filter(|n| n.in_degree == 0).cloned().collect();
    nodes.retain(|n| n.in_degree > 0);
    while zeros.len() > 0 {
        let removed = zeros.pop().unwrap();

        // decrease in-degree of zero's deps
        let dep = removed.dep_on.to_string();
        for t in dep.chars() {
            match nodes.iter_mut().find(|n| n.tag == t) {
                Some(n) => n.in_degree -= 1,
                None => {}
            }
        }

        // collect more zeros
        let mut more_zeros: Vec<Node> =
            nodes.iter().filter(|n| n.in_degree == 0).cloned().collect();
        nodes.retain(|n| n.in_degree > 0);
        zeros.append(&mut more_zeros);
        res.push(removed.tag);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn s1_works() {
        let nodes = vec![
            Node::new('0', "13"),
            Node::new('1', "23"),
            Node::new('3', "45"),
            Node::new('5', ""),
            Node::new('2', "345"),
            Node::new('4', "5"),
        ];
        let sequence = t_sort1(nodes);
        assert_eq!(sequence, "012345");
    }
    #[test]
    fn s2_works() {
        let nodes = vec![
            Node::new('0', "13"),
            Node::new('1', "23"),
            Node::new('3', "45"),
            Node::new('5', ""),
            Node::new('2', "345"),
            Node::new('4', "5"),
        ];
        let sequence = t_sort2(nodes);
        assert_eq!(sequence, "012345");
    }
}
