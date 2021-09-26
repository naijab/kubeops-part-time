use std::collections::HashMap;

pub fn spend_money(group: HashMap<String, i32>) -> HashMap<String, i32> {
    let mut result: HashMap<String, i32> = HashMap::new();

    let total_man: i32 = group.keys().len() as i32;
    let total_money: i32 = group.clone().into_iter().map(|(_id, money)| money).sum();
    let must_spend = (total_money / total_man) as i32;

    for (man, money) in &group {
        result.insert(man.to_string(), money - must_spend);
    }

    return result;
}

#[cfg(test)]
mod spend_money_test {
    use super::*;

    #[test]
    fn it_should_return_spend_money() {
        let mut group: HashMap<String, i32> = HashMap::new();
        group.insert("A".to_string(), 20);
        group.insert("B".to_string(), 15);
        group.insert("C".to_string(), 10);

        let actual = spend_money(group);

        let mut expect: HashMap<String, i32> = HashMap::new();
        expect.insert("A".to_string(), 5);
        expect.insert("B".to_string(), 0);
        expect.insert("C".to_string(), -5);

        assert_eq!(actual, expect);
    }
}
