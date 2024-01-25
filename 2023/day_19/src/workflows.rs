use crate::part::Part;
use crate::part_range::PartRange;

#[derive(Debug)]
pub struct Workflows<'a, const N: usize> {
    data: [Workflow<'a>; N],
    entry: usize,
    accept: Vec<PartRange>,
}

impl<'a, const N: usize> TryFrom<&'a str> for Workflows<'a, N> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut entry = 0;
        let mut v: Vec<Workflow<'a>> = Vec::with_capacity(N);
        let mut indexes = Vec::with_capacity(N);
        for (idx, wf) in value.lines().enumerate() {
            let mut wf: Workflow<'_> = wf.try_into()?;
            wf.name.index = idx;
            if wf.name.name == "in" {
                entry = idx;
            }
            indexes.push(wf.name);
            v.push(wf);
        }
        indexes.sort_unstable_by_key(|node| node.name);
        // Add the indexes to the default rules to that jump to other nodes
        // As opposed to the Accept / Reject sinks for which it is not relevant
        v.iter_mut()
            .map(|wf| &mut wf.default)
            .filter_map(|n| match n {
                NextNode::Node(x) => Some(x),
                _ => None,
            })
            .for_each(|n| {
                let posn = indexes
                    .binary_search_by_key(&n.name, |node| node.name)
                    .unwrap();
                n.index = indexes[posn].index;
            });
        // Add the indexes to the conditions rules to that jump to other nodes
        // As opposed to the Accept / Reject sinks for which it is not relevant
        v.iter_mut()
            .flat_map(|wf| &mut wf.conditions)
            .filter_map(|c| c.as_mut().map(|c| &mut c.next))
            .filter_map(|n| match n {
                NextNode::Node(x) => Some(x),
                _ => None,
            })
            .for_each(|n| {
                let posn = indexes
                    .binary_search_by_key(&n.name, |node| node.name)
                    .unwrap();
                n.index = indexes[posn].index;
            });
        let data = v.try_into().map_err(|_| {
            "Cannot convert the vector of Workflows into a fixed size array".to_string()
        })?;
        Ok(Self {
            data,
            entry,
            accept: Vec::new(),
        })
    }
}

impl<'a, const N: usize> Workflows<'a, N> {
    pub fn build_ranges(&mut self) {
        Self::build_ranges_inner(
            PartRange::new(),
            &self.data[self.entry],
            &self.data,
            &mut self.accept,
        );
    }

    fn build_ranges_inner(
        mut range: PartRange,
        wf: &Workflow<'a>,
        data: &[Workflow<'a>],
        accept: &mut Vec<PartRange>,
    ) {
        for condition in wf.conditions.iter().filter_map(|&c| c) {
            let (if_true, if_false) = range.apply(condition.test);
            if let Some(true_range) = if_true {
                match condition.next {
                    NextNode::Accept => accept.push(true_range),
                    NextNode::Reject => (),
                    NextNode::Node(n) => {
                        Self::build_ranges_inner(true_range, &data[n.index], data, accept);
                    }
                }
            }
            if if_false.is_none() {
                return;
            }
            range = if_false.unwrap();
        }
        match wf.default {
            NextNode::Accept => accept.push(range),
            NextNode::Reject => (),
            NextNode::Node(n) => {
                Self::build_ranges_inner(range, &data[n.index], data, accept);
            }
        }
    }

    pub fn accepted(&self, part: Part) -> bool {
        self.accept.iter().any(|r| r.within(part))
    }

    pub fn combinations(&self) -> u64 {
        self.accept.iter().map(PartRange::combinations).sum()
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: WorkflowNode<'a>,
    default: NextNode<'a>,
    conditions: [Option<Condition<'a>>; 3],
}

impl<'a> TryFrom<&'a str> for Workflow<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (name, rest) = value
            .split_once('{')
            .ok_or_else(|| format!("The workflow is missing an opening curly brace: {value}"))?;
        let mut conditions: [Option<Condition<'_>>; 3] = [None, None, None];
        let mut stub = None; // Needed so we can peek beyond the three conditions of the array
        let mut parts = rest
            .trim_end_matches('}')
            .split(',')
            .zip(conditions.iter_mut().chain(std::iter::once(&mut stub)))
            .peekable();
        while let Some((condition, next)) = parts.next() {
            if parts.peek().is_none() {
                let default = condition.into();
                if conditions
                    .iter()
                    .filter_map(|&c| c)
                    .all(|c| c.next == default)
                {
                    conditions = [None, None, None];
                }
                return Ok(Self {
                    name: name.into(),
                    default,
                    conditions,
                });
            }
            *next = Some(condition.try_into()?);
        }
        Err("Should have retuned???".to_owned())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Condition<'a> {
    test: ConditionTest,
    next: NextNode<'a>,
}

impl<'a> TryFrom<&'a str> for Condition<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (category, rest) = value.split_at(1);
        let (bound, rest) = rest.split_at(1);
        let (val, next) = rest.split_once(':').ok_or_else(|| format!("Missing the expected ':' delimiter to split the condition from the next workflow: {value}"))?;

        let category = category.into();
        let bound = bound.into();
        let val = val.parse().map_err(|_| {
            format!("Cannot convert the value part of the condition into a number: {value}")
        })?;

        Ok(Self {
            test: (category, bound, val).into(),
            next: next.into(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConditionTest {
    pub(crate) category: PartCategory,
    pub(crate) bound: Bound,
    pub(crate) val: u32,
}

impl From<(PartCategory, Bound, u32)> for ConditionTest {
    fn from(value: (PartCategory, Bound, u32)) -> Self {
        Self {
            category: value.0,
            bound: value.1,
            val: value.2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NextNode<'a> {
    Accept,
    Reject,
    Node(WorkflowNode<'a>),
}

impl<'a> From<&'a str> for NextNode<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            x => Self::Node(x.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WorkflowNode<'a> {
    name: &'a str,
    index: usize,
}

impl<'a> From<&'a str> for WorkflowNode<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            name: value,
            index: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound {
    Less,
    Greater,
}

impl From<&str> for Bound {
    fn from(value: &str) -> Self {
        match value {
            "<" => Self::Less,
            ">" => Self::Greater,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartCategory {
    ExtremelyCool,
    Musical,
    Aerodynamical,
    Shiny,
}

impl From<&str> for PartCategory {
    fn from(value: &str) -> Self {
        match value {
            "x" => Self::ExtremelyCool,
            "m" => Self::Musical,
            "a" => Self::Aerodynamical,
            "s" => Self::Shiny,
            _ => unreachable!(),
        }
    }
}
