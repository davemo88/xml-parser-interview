// # XML Parsing into Tree
// You are given a string representing XML text. Write a function in the language of your choice that takes the XML text as input and builds a tree structure representing the hierarchy of XML elements.
// Function Signature
// 
//  builds a tree structure  representing the hierarchy of XML
// 
// ```python
// def parse_xml_to_tree(xml_text: str) -> XmlNode:
//     ...
// ```
// 
// ## Input
// The function `parse_xml_to_tree` takes a single parameter `xml_text`, which is a string representing the XML text to be parsed.
// 
// ## Output
// 
// The function should return a `XmlNode` object representing the root of the constructed XML tree. Each node in the tree should store the tag name and any attributes associated with it.
// 
// 
// `XmlNode` object representing the root of the constructed XML tree.
// tag name and any attributes
// 
// ### XML Text Format
// The XML text follows the standard XML format, where elements are enclosed in tags and may have attributes. The structure of the XML is such that each element has only one parent and can have multiple children.
// 
// Here is an example XML text:
// ```xml
// <root>
//     <element1>
//         <subelement1/>
//         <subelement2>
//             <subsubelement/>
//         </subelement2>
//     </element1>
//     <element2>
//         <subelement3/>
//     </element2>
// </root>
// ```
// ### XmlNode Class Definition
// You can use the following ﻿TreeNode class definition or create your own as per your language of choice:
// 
// ```python
// class XmlNode:
//     def __init__(self, tag: str, attributes: dict):
//         self.children = []
//         self.parent = None
// ```
// 
// ## Example
// ### Input
// ```python
// xml_text = '<root><element1><subelement1/><subelement2 attribute3="value3"><subsubelement/></subelement2></element1><element2><subelement3/></element2></root>'
// ```
// ### Output
// 
// A Tree structure representing the XML hierarchy would be constructed, where the root node has the ﻿tag set to ﻿"root". The tree could be visualized as follows:
// ```
// root
// ├── element1
// │   ├── subelement1
// │   └── subelement2
// │       ├── subsubelement
// └── element2
//     └── subelement3
// ```
// ### Note
// You can assume that the given XML text will be well-formed and valid.
// The order of children under a parent element should be preserved.
// 
// Remember to include clear instructions, hints, and test cases in your coding interview question. Good luck!
// 
// 
// <tag1 attr="<<<>>>>>"/><dfs></dfs>
// 0      5    8
// tag_regex = '(<[^>]+>)'

use std::{cell::RefCell, rc::Rc};

use regex::Regex;
fn main() {
    println!("Hello, world!");
}
const EXAMPLE: &str = "<root><element1><subelement1/><subelement2 attribute3=\"value3\"><subsubelement/></subelement2></element1><element2><subelement3/></element2></root>";

#[derive(Debug)]
enum TagKind {
    Open,
    Close,
    SelfClosing
}

impl From<&str> for TagKind {
    fn from(value: &str) -> Self {
        for (i, char) in value.chars().enumerate() {
            if char == '/' && i == 1 {
                return Self::Close
            } else if char == '/' && i == value.len()-2 {
                return Self::SelfClosing
            }
        }
        Self::Open
    } 
}

type SharedNode = Rc<RefCell<XmlNode>>;

#[derive(Clone)]
struct XmlNode {
    pub tag: String,
    pub parent: Option<SharedNode>,
    pub children: Vec<SharedNode>,
}

impl std::fmt::Display for XmlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("node: {}\n", self.tag))?;
        f.write_str(&format!("parent: {}\n", self.parent.as_ref().map(|p| p.borrow().clone().tag.clone()).unwrap_or_default()))?;
        f.write_str("children: ")?;
        for c in self.children.iter() {
            f.write_str(&format!("\n{}", c.borrow()))?;
        }
        f.write_str("\n----")
    }
}

impl XmlNode {
    fn new(tag: String, parent: Option<SharedNode>) -> Self {
        Self {
            tag,
            parent,
            children: vec!(),
        }
    }

    fn shared(tag:String, parent: Option<SharedNode>) -> SharedNode {
        Rc::new(RefCell::new(Self::new(tag, parent)))
    }
}

fn to_tags(s: &str) -> Vec<String> {
    let r = Regex::new("(<[^>]+>)").unwrap();
    r.captures_iter(s)
        .map(|capture| capture.get(0).unwrap().as_str().to_string())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn hello_world() {
        println!("Hello, world!");
    }

    #[test]
    fn test_parse_xml() {
        let r = Regex::new("(<[^>]+>)").unwrap();
        let matches = r.captures_iter(EXAMPLE).collect::<Vec<_>>();
        for m in matches {
            let text = m.get(0).unwrap().as_str();
            println!("{text}");
        }
    }

    #[test]
    fn test_to_tag_kind() {
        let tags = to_tags(EXAMPLE);
        println!("{EXAMPLE}");
        for tag in tags {
            let tagkind = TagKind::from(tag.as_str());
            println!("{tagkind:?}")
        }
    }

    #[test]
    fn test_build_tree() {
        let tags = to_tags(EXAMPLE);
        let mut stack: Vec<SharedNode> = vec!();
        for tag in tags {
            match TagKind::from(tag.as_str()) {
                TagKind::Open => {
                    let parent = stack.last().cloned();
                    let node = XmlNode::shared(tag, parent.clone());
                    if let Some(p) = parent {
                        p.borrow_mut().children.push(node.clone());
                    }
                    stack.push(node);
                },
                TagKind::Close => {
                     let el = stack.pop();
                     if let Some(el) = el.clone() {
                         let el = el.borrow();
                         if el.tag == "<root>" {
                            println!("{el}");
                         }
                     }
                     if el.clone().unwrap().borrow().tag == "root" {
                         return;
                     }
                },
                TagKind::SelfClosing => {
                    let parent = stack.last().cloned().unwrap();
                    let node = XmlNode::shared(tag, stack.last().cloned());
                    parent.borrow_mut().children.push(node);
                },
            }
        }
    }
}
