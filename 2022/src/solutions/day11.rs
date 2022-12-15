use std::{io::{BufReader, Read}, fs::File, collections::HashMap, cell::{RefCell, RefMut}};

use itertools::Itertools;
use regex::Regex;


#[derive(Debug)]
#[derive(Clone, Copy)]
enum Value {
    CONST(i64),
    OLD   
}

impl Value {
    pub fn new (val: &str) -> Value {
        if val.trim() == "old" {
            Self::OLD
        } else {
            Self::CONST(val.trim().parse().unwrap())
        }
    }

    pub fn getVal (self: &Self, curr: i64) -> i64 {
        match self {
            Self::CONST(x) => *x,
            Self::OLD => curr
        }
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
enum Operation {
    ADD(Value),
    MULT(Value)
}

impl Operation {
    pub fn new (opStr: &str, opNum: &str) -> Operation {
        match opStr {
            "+" => Operation::ADD(Value::new(opNum)),
            "*" => Operation::MULT(Value::new(opNum)),
            _ => panic!("Invalid operation!")
        }
    }

    pub fn apply (self: &Self, curr: i64) -> i64 {
        match self {
            Operation::ADD(x) => curr + x.getVal(curr),
            Operation::MULT(x) => curr * x.getVal(curr)
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Monkey {
    num: i32,
    items: Vec<i64>,
    op: Operation,
    test: i64,
    trueMonkey: i32,
    falseMonkey: i32,
    inspectNum: i64
}

impl Monkey {
    pub fn new (num: i32, items: Vec<i64>, op: Operation, test: i64, trueMonkey: i32, falseMonkey: i32) -> Monkey {
        Monkey { 
            num, 
            items, 
            op, 
            test, 
            trueMonkey, 
            falseMonkey,
            inspectNum: 0 
        }
    }

    pub fn addItem (self: &mut Self, item: i64) {
        self.items.push(item);
    }

    pub fn takeTurn (self: &mut Self, system: &MonkeySystem, divWorry: bool, modulus: i64) {
        for i in &self.items {
            self.inspectNum += 1;

            let mut worryLvl = self.op.apply(*i);

            if divWorry {
                worryLvl /= 3;
            } else {
                worryLvl %= modulus;
            }

            if worryLvl % self.test == 0 {
                system.getMonkey(self.trueMonkey).addItem(worryLvl);
            } else {
                system.getMonkey(self.falseMonkey).addItem(worryLvl);
            }
        }

        self.items.clear();
    }

    pub fn getInspectNum (self: &Self) -> i64 {
        self.inspectNum
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct MonkeySystem {
    monkeys: HashMap<i32, RefCell<Monkey>>,
    modulus: i64
}

impl MonkeySystem {
    pub fn new () -> MonkeySystem {
        MonkeySystem { 
            monkeys: HashMap::new(),
            modulus: 1 
        }
    }

    pub fn addMonkey (self: &mut Self, monkey: Monkey) {
        self.modulus *= monkey.test;
        self.monkeys.insert(monkey.num, RefCell::new(monkey));
    }

    pub fn getMonkey (self: &Self, num: i32) -> RefMut<Monkey> {
        self.monkeys.get(&num).unwrap().borrow_mut()
    }

    pub fn takeRound (self: &Self, divWorry: bool) {
        let keys: Vec<i32> = self.monkeys.keys().sorted().map(|i| *i).collect();

        for i in keys {
            let mut monkey = self.getMonkey(i);
            monkey.takeTurn(&self, divWorry, self.modulus);
        }
    }

    pub fn getMonkeyBusiness (self: &Self) -> i64 {
        self.monkeys.values()
            .map(|r| r.borrow().getInspectNum())
            .sorted()
            .rev()
            .take(2)
            .reduce(|a, b| a * b)
            .unwrap()
    }
}

// regex: [^\n]*(\d+):\n\W*[^\d]*((?:\d+,? ?)*)\n\W*[^\n]*(.) (\d+|old)\n[^\d]*(\d+)\n.*(\d+)\n.*(\d+)\n

pub fn solve (mut file: BufReader<File>) {
    let pattern = Regex::new(r"[^\n]*(\d+):\n\W*[^\d]*((?:\d+,? ?)*)\n\W*[^\n]*(.) (\d+|old)\n[^\d]*(\d+)\n.*(\d+)\n.*(\d+)\n").unwrap();
    let mut fileStr = String::new();
    file.read_to_string(&mut fileStr).unwrap();

    let mut monkeySystem = MonkeySystem::new();

    for i in pattern.captures_iter(&fileStr) {
        let monkeyNum: i32 = i.get(1).unwrap().as_str().parse().unwrap();
        println!("{}", i.get(2).unwrap().as_str());
        let items: Vec<i64> = i.get(2).unwrap().as_str().split(",").map(|s| s.trim().parse::<i64>()).filter_map(Result::ok).collect();
        dbg!(&items);

        let opStr = i.get(3).unwrap().as_str();
        let opNum: &str = i.get(4).unwrap().as_str();

        let testVal: i64 = i.get(5).unwrap().as_str().parse().unwrap();

        let trueMonkey: i32 = i.get(6).unwrap().as_str().parse().unwrap();
        let falseMonkey: i32 = i.get(7).unwrap().as_str().parse().unwrap();

        let monkey = Monkey::new(monkeyNum, items, Operation::new(opStr, opNum), testVal, trueMonkey, falseMonkey);
        monkeySystem.addMonkey(monkey);
    } 

    let system2: MonkeySystem = monkeySystem.clone();

    dbg!(&monkeySystem);

    for _ in 0..20 {
        monkeySystem.takeRound(true);
    }

    for _ in 0..10000 {
        system2.takeRound(false);
    }

    println!("Monkey business: {}", monkeySystem.getMonkeyBusiness());
    println!("Monkey business 2: {}", system2.getMonkeyBusiness());
}