#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn partiallist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    let mut i = 0;
    while i < _second_list.len() {
        let mut j = 0;
        let start = i;
        while j < _first_list.len() && 
            i < _second_list.len() && _first_list[j] == _second_list[i] {
               i+=1;
               j+=1; 
            }
        if j == _first_list.len() {
            return true;
        }
        i = start + 1;
    }
    return false;
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        for i in 0.._first_list.len() {
            if _first_list[i] != _second_list[i] {
                return Comparison::Unequal;
            }
        }
        return Comparison::Equal;
    } else if _first_list.len() < _second_list.len() {
        if partiallist(_first_list, _second_list) {
            return Comparison::Sublist;
        }
    } else {
        if partiallist(_second_list, _first_list) {
            return  Comparison::Superlist;
        }
    }
    return Comparison::Unequal;
}
