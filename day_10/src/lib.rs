enum INST {
    NONE,
    NOOP,
    ADDX,
}
struct InsExec {instruction: INST, cycle_remaining: u8, data: Option<i32>}
struct Cpu {cycle: u32, x: i32, instruction: InsExec} 


pub fn part_one(input: &String) -> i32 {
    let mut res = 0;
    let mut end_of_exec = false;
    let inst_exec = InsExec {instruction: INST::NONE, cycle_remaining: 0, data: None};
    let mut cpu = Cpu {cycle: 1, x: 1, instruction: inst_exec};
    let mut instructions = input.lines().flat_map(|l| l.split("\n").collect::<Vec<_>>()).rev().collect::<Vec<_>>();
    
    while !end_of_exec {
        if is_cpu_idle(&cpu) {
            load_instruction(instructions.pop().unwrap(), &mut cpu);
            //instructions.drain(0..1);
        }
        exec(&mut cpu);
        if cpu.cycle == 20 {
            res += cpu.x * 20;
            println!("cycle={} value={}", cpu.cycle, cpu.cycle as i32 * cpu.x);
        } else if cpu.cycle > 20 && cpu.cycle <= 220 && (cpu.cycle - 20) % 40 == 0 {
            res += cpu.x * cpu.cycle as i32;
            println!("cycle={} value={}", cpu.cycle, cpu.cycle as i32 * cpu.x);
        }
if cpu.cycle == 219 {
    print!(" ");
}
        if instructions.is_empty() && is_cpu_idle(&cpu) {
            end_of_exec = true;
        }
    }

    res
}

fn is_cpu_idle(cpu: &Cpu) -> bool {
    cpu.instruction.cycle_remaining == 0
}

fn load_instruction(fetched_inst: &str, cpu: &mut Cpu) {
    
    let fetched_inst = fetched_inst.split(" ").collect::<Vec<_>>();
    match fetched_inst[0] {
        "noop" => {
            let inst_exec = InsExec {instruction: INST::NOOP, cycle_remaining: 0, data: None};
            cpu.instruction = inst_exec;
        },
        "addx" => {
            let inst_exec = InsExec {instruction: INST::ADDX, cycle_remaining: 2, data: Some(fetched_inst[1].parse::<i32>().unwrap())};
            cpu.instruction = inst_exec;
        },        
        _ => (),
    };
}


fn exec(cpu: &mut Cpu) {
    let exec_inst = &mut cpu.instruction;
    match exec_inst.instruction {
        INST::ADDX => {
            if exec_inst.cycle_remaining == 1 {
                cpu.x += exec_inst.data.unwrap();
            }
            exec_inst.cycle_remaining -= 1;
        },
        _ => (),
    }
    cpu.cycle += 1;
}





pub fn part_two(input: &String) -> i32 {
    let res = 0;
    let mut end_of_exec = false;
    let inst_exec = InsExec {instruction: INST::NONE, cycle_remaining: 0, data: None};
    let mut cpu = Cpu {cycle: 1, x: 1, instruction: inst_exec};
    let mut instructions = input.lines().flat_map(|l| l.split("\n").collect::<Vec<_>>()).rev().collect::<Vec<_>>();
    let mut crt = vec![" "; 240];

    while !end_of_exec {
        crt[cpu.cycle as usize - 1] = if fix_cycle(cpu.cycle) == cpu.x || fix_cycle(cpu.cycle) == cpu.x-1 || fix_cycle(cpu.cycle) == cpu.x+1 {"#"} else {"."};

        if is_cpu_idle(&cpu) {
            load_instruction(instructions.pop().unwrap(), &mut cpu);
            //instructions.drain(0..1);
        }
        exec(&mut cpu);

        if instructions.is_empty() || cpu.cycle == 241 {
            end_of_exec = true;
        }
    }

    for i in 1..=240 {
        print!("{}",crt[i-1]);
        if i % 40 == 0 {
            println!("");
        }
    }
    println!("");
    res
}

fn fix_cycle(cycle: u32) -> i32 {
    (cycle as i32 -1) % 40 
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"; 
    #[test]
    fn part_one_001() {
        let res = part_one(&INPUT.to_string());
        assert_eq!(13140, res);
    }
    #[test]
    fn part_two_001() {
        let res = part_two(&INPUT.to_string());
        assert_eq!(0, res);
    }
}
