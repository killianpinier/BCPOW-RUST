use blockchain_pow::app::App;


fn main() {
    let mut app = App::new(String::from("bitcoin"));
    app.run();
}
