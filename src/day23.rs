use nom::lib::std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Cup {
    number: usize,
    next: Option<Rc<RefCell<Cup>>>,
}

impl Cup {
    fn new(number: usize) -> Self {
        Self { number, next: None }
    }

    fn detach_after(&mut self, how_many: usize) -> Rc<RefCell<Cup>> {
        let first_detached_cup = self.next.take().unwrap();
        let mut next_cup = first_detached_cup.borrow().next.as_ref().unwrap().clone();
        for _ in 0..how_many - 2 {
            let next_next_cup = next_cup.borrow().next.as_ref().unwrap().clone();
            next_cup = next_next_cup;
        }
        let cup_after_detached_section = next_cup.borrow_mut().next.take().unwrap();
        self.next = Some(cup_after_detached_section);
        first_detached_cup
    }

    fn insert_after(&mut self, first_cup_to_insert: Rc<RefCell<Cup>>) {
        let cup_after_insertion = self.next.take().unwrap();
        let mut next_cup = first_cup_to_insert.clone();
        loop {
            if next_cup.borrow().next.is_none() {
                break;
            }
            let next_next_cup = next_cup.borrow().next.as_ref().unwrap().clone();
            next_cup = next_next_cup;
        }
        next_cup.borrow_mut().next = Some(cup_after_insertion);
        self.next = Some(first_cup_to_insert);
    }

    fn contains_number(&self, number: usize) -> bool {
        if self.number == number {
            true
        } else if let Some(next_cup) = self.next.as_ref() {
            next_cup.borrow().contains_number(number)
        } else {
            false
        }
    }
}

impl std::hash::Hash for Cup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.hash(state)
    }
}

type CupRef = Rc<RefCell<Cup>>;

fn build_cups(numbers: &[usize], number_of_cups: usize) -> Vec<CupRef> {
    let mut cups: HashMap<usize, CupRef> = HashMap::with_capacity(number_of_cups + 1);

    let first_cup = Rc::new(RefCell::new(Cup::new(numbers[0])));
    cups.insert(numbers[0], first_cup.clone());
    let mut previous_cup = first_cup.clone();

    for &num in numbers.iter().skip(1) {
        let cup = Rc::new(RefCell::new(Cup::new(num)));
        previous_cup.borrow_mut().next = Some(cup.clone());
        cups.insert(num, cup.clone());
        previous_cup = cup
    }

    for num in numbers.len() + 1..=number_of_cups {
        let cup = Rc::new(RefCell::new(Cup::new(num)));
        previous_cup.borrow_mut().next = Some(cup.clone());
        cups.insert(num, cup.clone());
        previous_cup = cup
    }

    previous_cup.borrow_mut().next = Some(first_cup);

    cups.insert(0, Rc::new(RefCell::new(Cup::new(0))));

    (0..=number_of_cups)
        .into_iter()
        .map(|num| cups.get(&num).unwrap().clone())
        .collect()
}

fn play_game(cups: &[Rc<RefCell<Cup>>], first_cup: usize, rounds: usize) {
    let mut current_cup = cups[first_cup].clone();
    for _ in 0..rounds {
        let detached_cups = current_cup.borrow_mut().detach_after(3);
        let destination_cup = &cups
            [find_valid_destination(current_cup.borrow().number, &detached_cups, cups.len() - 1)];
        destination_cup.borrow_mut().insert_after(detached_cups);
        let next_cup = current_cup.borrow().next.as_ref().unwrap().clone();
        current_cup = next_cup;
    }
}

fn find_valid_destination(
    current_cup_num: usize,
    detached_cups: &Rc<RefCell<Cup>>,
    num_cups: usize,
) -> usize {
    let mut destination_cup_num = current_cup_num - 1;
    if destination_cup_num == 0 {
        destination_cup_num = num_cups;
    }
    while detached_cups.borrow().contains_number(destination_cup_num) {
        destination_cup_num -= 1;
        if destination_cup_num == 0 {
            destination_cup_num = num_cups;
        }
    }
    destination_cup_num
}

fn get_cup_labels_after_1(cups: &[Rc<RefCell<Cup>>]) -> u64 {
    let mut current_cup = cups[1].clone();
    let mut labels = 0u64;
    for _ in 0..cups.len() - 1 {
        let next_cup = current_cup.borrow().next.as_ref().unwrap().clone();
        current_cup = next_cup;
        labels *= 10;
        labels += current_cup.borrow().number as u64;
    }
    labels
}

pub fn solution(input: String) -> [u64; 2] {
    let numbers = input
        .chars()
        .map(|s| s.to_string().parse().unwrap())
        .collect::<Vec<usize>>();
    let part1_cups = build_cups(&numbers, numbers.len());

    play_game(&part1_cups, numbers[0], 100);
    let part1_answer = get_cup_labels_after_1(&part1_cups);

    let part2_cups = build_cups(&numbers, 1000000);

    play_game(&part2_cups, numbers[0], 10000000);
    let next_cup_1 = part2_cups[1].borrow().next.as_ref().unwrap().clone();
    let next_cup_2 = next_cup_1.borrow().next.as_ref().unwrap().clone();
    let part2_answer = (next_cup_1.borrow().number * next_cup_2.borrow().number) as u64;

    [part1_answer, part2_answer]
}
