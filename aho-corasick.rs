use std::vec;
type State = usize;
const ROOT_STATE: State = 0;
type CallbackMut = dyn Fn(&mut TrieNode, &str);

#[derive(Debug, Clone)]
struct TrieNode {
    label: char,
    state: State,
    // longest common prefix
    fail_to: State,
    children: Vec<TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            // `Option` would be messy
            label: ' ',
            state: ROOT_STATE,
            fail_to: ROOT_STATE,
            children: vec![],
        }
    }
    fn find_lcp(&self, prefix: &str, exclude: State) -> State {
        let mut result = 0;
        let mut longest = 0;
        traverse(
            self,
            &mut |state, pre| {
                if prefix.ends_with(pre) && pre.len() > longest && state != exclude {
                    longest = pre.len();
                    result = state;
                }
            },
            String::new(),
        );
        result
    }
    fn can_goto(&self, label: &char) -> Option<&TrieNode> {
        self.children.iter().find(|c| &c.label == label)
    }
}
fn traverse_mut(n: &mut TrieNode, callback: &CallbackMut, prefix: String) {
    for c in n.children.iter_mut() {
        let mut new_prefix = prefix.clone();
        new_prefix.push(c.label);
        callback(c, &new_prefix);
        traverse_mut(c, callback, new_prefix);
    }
}
fn traverse(n: &TrieNode, callback: &mut impl FnMut(usize, &str), prefix: String) {
    for c in n.children.iter() {
        let mut new_prefix = prefix.clone();
        new_prefix.push(c.label);
        callback(c.state, &new_prefix);
        traverse(c, callback, new_prefix);
    }
}
#[derive(Debug)]
struct AC {
    // redundance for conveniency
    all_char: String,
    state_root: TrieNode,
    output: Vec<State>,
}
impl AC {
    fn new() -> Self {
        Self {
            state_root: TrieNode::new(),
            all_char: String::new(),
            output: vec![],
        }
    }
    fn build(&mut self, keywords: Vec<&str>) {
        self.build_goto(keywords);
        self.build_fail();
    }
    fn build_goto(&mut self, keywords: Vec<&str>) {
        for kw in keywords {
            // dbg!(&kw, &self);
            let mut n = &mut self.state_root;
            let mut s = ROOT_STATE;
            for (i, ch) in kw.chars().enumerate() {
                match n.children.iter().enumerate().find(|(_, c)| c.label == ch) {
                    Some((j, _)) => {
                        // moving to a exist branch
                        n = &mut n.children[j];
                        s = n.state;
                    }
                    None => {
                        // create new branch
                        let mut new_nodes = vec![];
                        for c in kw[i..].chars() {
                            self.all_char.push(c);
                            let n = TrieNode {
                                label: c,
                                state: self.all_char.len(),
                                fail_to: ROOT_STATE,
                                children: vec![],
                            };
                            s = n.state;
                            new_nodes.push(n);
                        }
                        while let Some(new_node) = new_nodes.pop() {
                            match new_nodes.last_mut() {
                                Some(last) => last.children.push(new_node),
                                None => n.children.push(new_node),
                            }
                        }
                        break;
                    }
                }
            }
            self.output.push(s);
            // println!("\n");
        }
    }
    fn build_fail(&mut self) {
        let cloned = self.state_root.clone();
        self.traverse_from_root(&move |n, s| {
            n.fail_to = cloned.find_lcp(s, n.state);
        });
    }
    fn traverse_from_root(&mut self, callback: &CallbackMut) {
        traverse_mut(&mut self.state_root, callback, String::new());
    }
    fn search(&self, source: &str) -> Vec<State> {
        let mut n = &self.state_root;
        let mut result = vec![];
        for ch in source.chars() {
            while n.state > ROOT_STATE && n.can_goto(&ch).is_none() {
                n = self.get_node(n.fail_to);
            }
            if self.output.contains(&n.state) {
                // fail to some output node (she -> he)
                // maybe
                result.push(n.state);
            }
            match n.can_goto(&ch) {
                Some(next) => {
                    n = next;
                    if self.output.contains(&n.state) {
                        result.push(n.state);
                    }
                }
                None => continue,
            }
        }
        result
    }
    fn get_node(&self, state: usize) -> &TrieNode {
        get(&self.state_root, state).unwrap()
    }
}
fn get(node: &TrieNode, state: State) -> Option<&TrieNode> {
    if node.state == state {
        Some(node)
    } else {
        for ch in &node.children {
            if let Some(n) = get(ch, state) {
                return Some(n);
            }
        }
        None
    }
}
pub fn ac_search(source: &str, keywords: Vec<&str>) -> Vec<State> {
    let mut ac = AC::new();
    ac.build(keywords);
    ac.search(source)
}
#[test]
fn test_prepare() {
    let mut machine = AC::new();
    machine.build(vec!["he", "she", "his", "hers"]);
    dbg!(&machine);
    assert_eq!(machine.all_char, "hesheisrs");
    for s in [2, 5, 7, 9] {
        assert!(machine.output.contains(&s));
    }
}
#[test]
fn test_search() {
    let result = ac_search("ushers", vec!["he", "she", "his", "hers"]);
    for s in [2, 5, 9] {
        assert!(result.contains(&s));
    }
}
