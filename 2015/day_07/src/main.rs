#![warn(clippy::all, clippy::pedantic)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> Signal {
    let mut c = new_circuit(data);
    eval_circuit(&mut c);
    get_gate(&c, "a").unwrap().value
}

fn part_two(data: &'static str) -> Signal {
    let mut c = new_circuit(data);
    let b = c.iter_mut().find(|g| g.name == "b").unwrap();
    b.op = Operator::Assign(Operand::Value(46_065));
    eval_circuit(&mut c);
    get_gate(&c, "a").unwrap().value
}

const EMPTY_NAME: &str = "";
type Signal = u16;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Assign(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
    Lshift(Operand, u32),
    Rshift(Operand, u32),
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Value(Signal),
    Gate(&'static str),
}

#[derive(Debug, Clone, Copy)]
struct Gate {
    name: &'static str,
    value: Signal,
    op: Operator,
}

impl Default for Gate {
    fn default() -> Self {
        Self {
            name: EMPTY_NAME,
            value: 0,
            op: Operator::Assign(Operand::Value(0)),
        }
    }
}

type Circuit = [Gate; 350];

impl Gate {
    fn new(line: &'static str) -> Self {
        let (operation, gate) = line.split_once(" -> ").unwrap();

        let mut parts = operation.split(' ');
        let (p1, p2, p3) = (parts.next(), parts.next(), parts.next());

        let op = match p1 {
            Some("NOT") => Operator::Not(to_operand(p2.unwrap())),
            Some(g) => {
                let op1 = to_operand(g);
                match p2 {
                    Some("AND") => Operator::And(op1, to_operand(p3.unwrap())),
                    Some("OR") => Operator::Or(op1, to_operand(p3.unwrap())),
                    Some("RSHIFT") => Operator::Rshift(op1, p3.unwrap().parse().unwrap()),
                    Some("LSHIFT") => Operator::Lshift(op1, p3.unwrap().parse().unwrap()),
                    Some(_) => unreachable!(),
                    None => Operator::Assign(op1),
                }
            }
            None => unreachable!(),
        };

        Self {
            name: gate,
            value: 0,
            op,
        }
    }
}

fn new_circuit(data: &'static str) -> Circuit {
    let mut circuit = [Gate::default(); 350];
    for (gate, line) in circuit.iter_mut().zip(data.lines()) {
        *gate = Gate::new(line);
    }
    sorted_circuit(circuit)
}

fn sorted_circuit(mut source: Circuit) -> Circuit {
    let mut out = [Gate::default(); 350];
    let mut count = 0;

    // Simple assigns first
    for gate in source.iter_mut().filter(|g| g.name != EMPTY_NAME) {
        if let Operator::Assign(Operand::Value(_)) = gate.op {
            out[count] = *gate;
            count += 1;
            gate.name = EMPTY_NAME;
        }
    }

    // Now loop incrementally adding fully dependent gates
    let mut loop_count = count;
    while loop_count != 0 {
        loop_count = 0;
        for gate in source.iter_mut().filter(|g| g.name != EMPTY_NAME) {
            match gate.op {
                Operator::Assign(a)
                | Operator::Not(a)
                | Operator::Rshift(a, _)
                | Operator::Lshift(a, _)
                    if circuit_contains(&out, a) =>
                {
                    out[count] = *gate;
                    count += 1;
                    loop_count += 1;
                    gate.name = EMPTY_NAME;
                }
                Operator::And(a, b) | Operator::Or(a, b)
                    if circuit_contains(&out, a) && circuit_contains(&out, b) =>
                {
                    out[count] = *gate;
                    count += 1;
                    loop_count += 1;
                    gate.name = EMPTY_NAME;
                }
                _ => (),
            }
        }
    }

    out
}

fn circuit_contains(circuit: &Circuit, op: Operand) -> bool {
    let gate_str = match op {
        Operand::Value(_) => return true,
        Operand::Gate(g) => g,
    };
    for gate in circuit.iter() {
        if gate.name == gate_str {
            return true;
        }
    }
    false
}

fn eval_circuit(circuit: &mut Circuit) {
    for i in 1..circuit.len() {
        let (sub, _) = circuit.split_at_mut(i);
        if let Some((head, data)) = sub.split_last_mut() {
            eval_gate(data, head);
        }
    }
}

fn eval_gate(circuit: &[Gate], gate: &mut Gate) {
    gate.value = match gate.op {
        Operator::Assign(a) => eval_operand(circuit, a),
        Operator::And(a, b) => eval_operand(circuit, a) & eval_operand(circuit, b),
        Operator::Or(a, b) => eval_operand(circuit, a) | eval_operand(circuit, b),
        Operator::Not(a) => !eval_operand(circuit, a),
        Operator::Rshift(a, sh) => eval_operand(circuit, a) >> sh,
        Operator::Lshift(a, sh) => eval_operand(circuit, a) << sh,
    }
}

fn eval_operand(circuit: &[Gate], op: Operand) -> Signal {
    match op {
        Operand::Value(v) => v,
        Operand::Gate(g) => get_gate(circuit, g).unwrap().value,
    }
}

fn get_gate<'a>(circuit: &'a [Gate], gate: &'static str) -> Option<&'a Gate> {
    circuit.iter().rev().find(|&g| g.name == gate)
}

fn to_operand(operand: &'static str) -> Operand {
    if is_gate(operand) {
        Operand::Gate(operand)
    } else {
        Operand::Value(operand.parse().unwrap())
    }
}

fn is_gate(operand: &'static str) -> bool {
    (b'a'..=b'z').contains(&operand.as_bytes()[0])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(0b1111_1111_1111_0110, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(19_582, part_two(data));
    }
}
