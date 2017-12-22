mod lib;

#[derive(Clone, Copy)]
enum Reg {
    Ref(char),
    Val(i64),
}

#[derive(Clone, Copy)]
enum Instr {
    Snd(Reg),
    Set(char, Reg),
    Add(char, Reg),
    Mul(char, Reg),
    Mod(char, Reg),
    Rcv(char),
    Jgz(Reg, Reg),
}

type Map = std::collections::HashMap<char,i64>;
type Queue = std::collections::VecDeque<i64>;

// env + special register for last sound played
type Env1 = (i64, Map);

type Env2 = ((Queue, Map), (Queue, Map));

#[derive(Clone, Copy, Debug)]
enum Process {
    Fst,
    Snd,
}

enum Effect1 {
    Jump(i64),
    Rcv(i64),
}

enum Effect2 {
    Jump(i64),
    Wait,
    Send,
}

fn parse_line_aux(line: &str) -> Option<Instr> {
    fn register(tok: &str) -> Option<Reg> {
        match tok.parse::<i64>() {
            Err(_) => Some(Reg::Ref(char(tok)?)),
            Ok(n) => Some(Reg::Val(n)),
        }
    }
    fn char(tok: &str) -> Option<char> {
        Some(tok.chars().nth(0)?)
    }
    let mut it = line.split(char::is_whitespace);
    let fst = it.next();
    let snd = it.next();
    let thd = it.next();
    match fst? {
        "snd" => Some(Instr::Snd(register(snd?)?)),
        "set" => Some(Instr::Set(char(snd?)?, register(thd?)?)),
        "add" => Some(Instr::Add(char(snd?)?, register(thd?)?)),
        "mul" => Some(Instr::Mul(char(snd?)?, register(thd?)?)),
        "mod" => Some(Instr::Mod(char(snd?)?, register(thd?)?)),
        "rcv" => Some(Instr::Rcv(char(snd?)?)),
        "jgz" => Some(Instr::Jgz(register(snd?)?, register(thd?)?)),
        _ => None,
    }
}

fn parse_line(line: &str) -> Result<Instr, Box<std::error::Error>> {
    parse_line_aux(line).ok_or(format!("parse_line: {}", line.clone()).into())
}

fn deref(env: &Map, reg: Reg) -> i64 {
    match reg {
        Reg::Ref(c) => *(env.get(&c).unwrap_or(&0)),
        Reg::Val(v) => v,
    }
}

fn update_with<K, F: FnOnce(i64,i64) -> (i64, K)>(env: &mut Map, var: char, reg: Reg, f: F) -> K {
    let val = deref(&env, reg);
    let slot = env.entry(var).or_insert(0);
    let (value, eff) = f(*slot, val);
    *slot = value;
    eff
}

fn eval1(env: &mut Env1, instr: Instr) -> Option<Effect1> {
    match instr {
        Instr::Snd(reg) => {
            env.0 = deref(&env.1, reg);
            None
        },
        Instr::Set(var, reg) => update_with(&mut env.1, var, reg, |_,y| (y,None)),
        Instr::Add(var, reg) => update_with(&mut env.1, var, reg, |x,y| (x + y, None)),
        Instr::Mul(var, reg) => update_with(&mut env.1, var, reg, |x,y| (x * y, None)),
        Instr::Mod(var, reg) => update_with(&mut env.1, var, reg, |x,y| (x % y, None)),
        Instr::Rcv(var) => if 0 == deref(&env.1, Reg::Ref(var)) {
            None
        } else {
            Some(Effect1::Rcv(env.0))
        },
        Instr::Jgz(reg1, reg2) => {
            if 0 < deref(&env.1, reg1) {
                Some(Effect1::Jump(deref(&env.1, reg2)))
            } else {
                None
            }
        },
    }
}

fn eval2(env: &mut Env2, process: Process, instr: Instr) -> Option<Effect2> {
    let (q, mut env, other_q) = match process {
        Process::Fst => (&mut (env.0).0, &mut (env.0).1, &mut (env.1).0),
        Process::Snd => (&mut (env.1).0, &mut (env.1).1, &mut (env.0).0),
    };
    match instr {
        Instr::Snd(reg) => {
            other_q.push_back(deref(&env, reg));
            Some(Effect2::Send)
        },
        Instr::Set(var, reg) => (update_with(&mut env, var, reg, |_,y| (y,None))),
        Instr::Add(var, reg) => (update_with(&mut env, var, reg, |x,y| (x + y, None))),
        Instr::Mul(var, reg) => (update_with(&mut env, var, reg, |x,y| (x * y, None))),
        Instr::Mod(var, reg) => (update_with(&mut env, var, reg, |x,y| (x % y, None))),
        Instr::Rcv(var) => match q.pop_front() {
            Some(val) => update_with(&mut env, var, Reg::Val(val), |_, y| (y, None)),
            None => Some(Effect2::Wait),
        },
        Instr::Jgz(reg1, reg2) => {
            if 0 < deref(&env, reg1) {
                Some(Effect2::Jump(deref(&env, reg2)))
            } else {
                None
            }
        },
    }
}

fn solve1(instrs: &Vec<Instr>) -> Option<i64> {
    let mut instr = 0;
    let mut env = Default::default();
    loop {
        if instr < 0 {
            break;
        }
        match instrs.get(instr as usize).cloned().map(|instr| eval1(&mut env, instr)) {
            None => break,
            Some(None) => instr += 1,
            Some(Some(Effect1::Jump(val))) => instr += val,
            Some(Some(Effect1::Rcv(rcv))) => return Some(rcv),
        }
    }
    None
}

fn solve2(instrs: &Vec<Instr>) -> i64 {
    let mut instr1 = 0;
    let mut instr2 = 0;
    let mut break1 = false;
    let mut break2 = false;
    let mut send_count = 0;
    let mut env : Env2 = Default::default();
    (env.1).1.insert('p', 1);
    fn update(env: &mut Env2, process: Process, instr: i64, instrs: &Vec<Instr>, break_: bool) -> (Option<Effect2>, bool) {
        if break_ || instr < 0 || instrs.len() <= instr as usize {
            (None, true)
        } else {
            (eval2(env, process, instrs[instr as usize]), false)
        }
    }
    while !break1 || !break2 {
        let (eff1, break1_) = update(&mut env, Process::Fst, instr1, &instrs, break1);
        let (eff2, break2_) = update(&mut env, Process::Snd, instr2, &instrs, break2);
        break1 = break1_;
        break2 = break2_;
        let mut wait1 = false;
        let mut wait2 = false;
        match eff1 {
            Some(Effect2::Jump(val)) => instr1 += val,
            Some(Effect2::Wait) => wait1 = true,
            Some(Effect2::Send) | None => instr1 += 1,
        }
        match eff2 {
            Some(Effect2::Jump(val)) => instr2 += val,
            Some(Effect2::Wait) => wait2 = true,
            Some(Effect2::Send) => {
                send_count += 1;
                instr2 += 1;
            },
            None => instr2 += 1,
        }
        if wait1 && wait2 {
            break;
        }
    }
    send_count
}



fn parse(input: &str) -> Result<Vec<Instr>, Box<std::error::Error>> {
    input.trim().lines().map(parse_line).collect()
}

fn main() {
    let instrs = parse(&lib::read_input_file().unwrap()).unwrap();
    println!("{}", solve1(&instrs).unwrap());
    println!("{}", solve2(&instrs));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let instrs = parse(r#"
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
"#).unwrap();
        assert_eq!(solve1(&instrs), Some(4));
    }

    #[test]
    fn test2() {
        let instrs = parse(r#"
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
"#).unwrap();
        assert_eq!(solve2(&instrs), 3);
    }
}
