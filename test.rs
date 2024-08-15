
pub struct Object {
    pub f: String,
}

impl Object {
    pub fn mutate(&mut self) {
        self.f = "something else".to_string();
    }
}

fn main() {

    let mut x = Object {
        f: "something".to_string(),
    };

    let func1 = || {
        x.mutate();
    };

    let func2 = || {
        x.mutate();
    };

    func1();
    func2();
    func1();
    func2();

}