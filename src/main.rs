#[derive(Clone)]
enum Address {
    Address(Box<List>),
    Nil
}
#[derive(Clone)]
struct List {
    value: u32,
    next: Address
}

impl List {
    fn append(&mut self, elem: u32) {
        match self.next {
            Address::Address(ref mut next_address) => {
                next_address.append(elem);
            }

            Address::Nil => {
                let node = List {
                    value: elem,
                    next: Address::Nil
                };
                self.next = Address::Address(Box::new(node));
            }
        }
    }

    fn delete(&mut self, elem: u32) {
        match self.next {
            Address::Address(ref mut next_address) => {
                if next_address.value == elem {
                    println!("Deleting address {}", next_address.value);
                    self.next = next_address.next.clone();
                }
            }
            Address::Nil => {
                if self.value == elem {
                    self.value = 0;
                } else {
                    println!("Element {} does not exist in the linked list", elem);
                }
            }
        }
    }

    fn count(&self) -> u32 {
        match self.next {
            Address::Address(ref newaddress) => 1 + newaddress.count(),
            Address::Nil => 0,
        }
    }
    
    fn list(&self) {
        if self.value == 0 {
            println!("The list is empty")
        } else {
            println!("{}", self.value);
            match self.next {
                Address::Address(ref next_address) => next_address.list(),
                Address::Nil => {}
            }
        }
    }

    fn update(&mut self, index: u32, elem: u32) {
        let mut i = 0;
        let mut j = self;
        if i == index {
            j.value = elem;
        }
        while i < index {
            match j.next {
                Address::Address(ref mut next_address) => j = next_address,
                Address::Nil => {}
            }
            i = i + 1;
        }
        j.value = elem;
    }
}

fn main() {
    let mut head = List {
        value: 1,
        next: Address::Nil,
    };
    head.append(2);
    head.append(3);
    head.append(4);
    head.list();
    println!("The size of the list is {}", head.count());
    
    head.update(4, 0);
    head.update(0, 4);
    head.list();
    
    head.delete(3);
    head.list();
}
