unsafe extern "C" {
    fn Install();
    fn Create();
    fn Start();
}

pub fn install() {
    unsafe { Install() }
}

pub fn create() {
    unsafe { Create() }
}

pub fn start() {
    unsafe { Start() }
}
