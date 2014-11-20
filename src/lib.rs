/// This scanner collapses an iterator into vectors separated by a delimiter
pub struct ScanDelim<T> {
    delim: T,
    data: Vec<T>,
}

impl<T: Eq + Clone> ScanDelim<T> {
    pub fn new(delim: T) -> ScanDelim<T> {
        ScanDelim { delim: delim, data: Vec::new() }
    }

    pub fn scanner(state: &mut ScanDelim<T>, val: T) -> Option<Vec<T>> {
        if val == state.delim {
            // let out: Vec<T> = Vec::new();
            // out.clone_from(&state.data);
            let out = state.data.clone();
            state.data.clear();
            Some(out)
        } else {
            state.data.push(val);
            None
        }
    }
}

#[test]
fn it_works() {
    
}
