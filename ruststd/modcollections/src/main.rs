use std::collections::BTreeMap;

fn main() {
    let mut count = BTreeMap::new();
    let message = "she sells sea shells by the sea shore";

    for c in message.chars() {
        *count
            .entry(c)
            .or_insert(0) += 1;
    }

    for (c, count) in count.iter() {
        println!("key: {}, count: {}", c, count);
    }

    //
    let mut blood_alcohol = BTreeMap::new();
    let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1];

    for id in orders {
        let person = blood_alcohol
            .entry(id)
            .or_insert(Person { blood_alcohol: 0.0 });
        person.blood_alcohol *= 0.9;

        if person.blood_alcohol > 0.3 {
            println!("sorry {id}, I have to cut you off");
        } else {
            person.blood_alcohol += 0.1;
        }
    }
}

struct Person {
    blood_alcohol: f32,
}
