use anyhow::{Context, Result};
use lazy_static::lazy_static;
use markdown::mdast::{Node, Text};
use markdown::message::Message;
use markdown::to_mdast;
use regex::Regex;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Missing arguments")]
    MissingArgument,

    #[error("Input doesn't satisfy assumed guarantees or is malformed")]
    BadInput,

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

// TODO: Contain info about the line that caused the initial error
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Empty Stream")]
    EmptyStream,
    #[error("No Text in Heading")]
    NoTextInHeading,
    #[error("Node not child node")]
    NodeNotDeepEnough,
    #[error("Expected Heading")]
    NotHeading(Node),
}

#[derive(Debug, Clone)]
pub struct TodoNode {
    status: TodoStatus,
    title: String,
    // body: Option<String>, // Implement this later
    scheduled: Option<String>, // Turn this into a date
    deadline: Option<String>,  // Turn this into a date
    // tags: Vec<String>, // Implement this later
    children: Vec<TodoNode>,
}

#[derive(Debug, Clone)]
enum TodoStatus {
    Unlabeled,
    Todo,
    Done,
    Waiting,
    Inactive,
    Canceled,
}

fn parse_markdown_node(markdown_node: &Node) -> Vec<TodoNode> {
    unimplemented!()
}

fn parse_todo_node(stream: &[Node], node_depth: u8) -> Result<(TodoNode, &[Node]), ParseError> {
    match stream {
        [] => Err(ParseError::EmptyStream),
        [Node::Heading(head), tail @ ..] => {
            let (contents, depth) = (head.children.as_slice(), head.depth);
            if depth < node_depth {
                return Err(ParseError::NodeNotDeepEnough);
            } else {
                match contents {
                    [Node::Text(text)] => {
                        let (status, title, scheduled, deadline) = parse_heading_text(text)?;
                        let tail = parse_todo_body(tail)?;
                        let mut children = Vec::new();

                        let tail = parse_children_nodes(tail, depth + 1, &mut children)?;

                        let todo_node = TodoNode {
                            status,
                            title,
                            scheduled,
                            deadline,
                            children,
                        };
                        Ok((todo_node, tail))
                    }
                    _ => Err(ParseError::NoTextInHeading),
                }
            }
        }
        [n, ..] => Err(ParseError::NotHeading(n.clone())),
    }
}

fn parse_todo_body(stream: &[Node]) -> Result<&[Node], ParseError> {
    match stream {
        [Node::Paragraph(_), tail @ ..] => parse_todo_body(tail),
        [Node::List(_), tail @ ..] => parse_todo_body(tail),
        _ => Ok(stream),
    }
}

fn parse_children_nodes<'a>(
    stream: &'a [Node],
    node_depth: u8,
    buffer: &mut Vec<TodoNode>,
) -> Result<&'a [Node], ParseError> {
    match parse_todo_node(stream, node_depth) {
        Ok((parsed_node, tail)) => {
            buffer.push(parsed_node);
            parse_children_nodes(tail, node_depth, buffer)
        }
        Err(ParseError::EmptyStream) => Ok(stream),
        Err(ParseError::NodeNotDeepEnough) => Ok(stream),
        Err(e) => Err(e),
    }
}

fn parse_heading_text(
    text: &Text,
) -> Result<(TodoStatus, String, Option<String>, Option<String>), ParseError> {
    lazy_static! {
        static ref STATUS: Regex =
            Regex::new(r"^\((TODO|WAITING|CANCELED|INACTIVE|DONE)\)").unwrap();
        static ref TITLE: Regex = Regex::new(r"^(\(.*?\))?\s*(.*?)\s*(\(.*\))?$").unwrap();
    }
    unimplemented!()
}

pub fn read_markdown_from_path(path: &Path) -> Result<Node, Message> {
    let contents = fs::read_to_string(path).expect("File not found");
    let parsed = to_mdast(&contents, &markdown::ParseOptions::mdx());
    parsed
}

pub fn indented_tree_print(parse_tree: &Node, depth: usize) {
    for i in parse_tree.children().unwrap() {
        println!("{:?}", i);
    }
}
