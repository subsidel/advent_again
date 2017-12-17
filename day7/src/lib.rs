#![feature(box_syntax)]
#![feature(box_patterns)]
#[cfg(test)]
mod tests {
    use ::*;
    const EXAMPLE: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn it_works() {
        assert_eq!(
            find_tree_base(parse_notes(EXAMPLE)),
            Some(String::from("tknk"))
        );
    }
    #[test]
    fn it_trees() {
        let lines = parse_notes(EXAMPLE);
        let base = find_tree_base(lines.clone());
        let tree = form_tree(&base.unwrap(), lines);
        println!("{:#?}", tree);
    }

}
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Line {
    weight: u32,
    children: Option<Vec<String>>,
    parent: Option<String>,
}

impl Line {
    pub fn change_parent(&self, parent: String) -> Line {
        Line {
            weight: self.weight,
            children: self.children.clone(),
            parent: Some(parent),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    weight: u32,
    pub children: Box<Option<Vec<Node>>>,
    parent: Box<Option<Node>>,
    pub disc_weight: u32,
}


pub fn parse_notes(notes: &str) -> HashMap<String, Line> {
    let mut lines_from_notes: HashMap<String, Line> = HashMap::new();
    for string_lines in notes.split('\n') {
        let parts: Vec<String> = string_lines
            .split_whitespace()
            .map(|s| String::from(s))
            .collect();
        let name = parts[0].clone();
        let weight = u32::from_str(&parts[1].trim_matches('(').trim_matches(')'));
        let children = parts.get(3..).and_then(|c| {
            Some(
                c.to_vec()
                    .iter()
                    .map(|s| String::from(s.trim_matches(',')))
                    .collect::<Vec<String>>(),
            )
        });
        lines_from_notes.insert(
            name,
            Line {
                weight: weight.unwrap(),
                children,
                parent: None,
            },
        );
    }
    lines_from_notes
}

pub fn find_tree_base(lines: HashMap<String, Line>) -> Option<String> {
    let mut parented_lines = HashMap::new();
    for (name, line) in lines.iter() {
        parented_lines.insert(name.clone(), line.clone());
    }

    for (name, line) in lines.iter() {
        if let Some(ref children) = line.children {
            for child in children.iter() {
                let new_child = lines.get(child).unwrap().change_parent(name.clone());
                parented_lines.insert(child.clone(), new_child);
            }
        }
    }

    let mut base: Option<String> = None;
    for (name, line) in parented_lines.iter() {
        if let None = line.parent {
            base = Some(name.clone());
        }
    }

    base
}

pub fn form_tree(base: &str, lines: HashMap<String, Line>) -> Node {
    let base_line = lines.get(base).unwrap();
    let children: Option<Vec<Node>> = base_line.clone().children.map(|ref children_lines| {
        children_lines
            .iter()
            .map(|child_line| form_tree(child_line, lines.clone()))
            .collect()
    });

    let disc_weight = match children {
        Some(ref children) => children
            .iter()
            .fold(base_line.weight, |acc, child| acc + child.disc_weight),
        None => base_line.weight,
    };

    Node {
        parent: box None,
        children: box children,
        weight: base_line.weight,
        disc_weight,
    }
}
