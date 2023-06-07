use gtest::{Log, Program, System};

#[test]
fn hello_test() {
    let sys = System::new();
    //inicicalizar los registros de impresion en stdout
    sys.init_logger();
    //let program = Program::current(&sys);
    let program = Program::from_file(&sys,
        "./target/wasm32-unknown-unknown/release/hello_world.wasm");

    program.send_bytes(2, String::from("Init Msg Sended"));

    /*
    assert!(res.log().is_empty());
    assert!(!res.main_failed());

    //send a new msg that handle will process
    let res = program.send(2, String::from("Second Msg Sended"));


    let expected_log = Log::builder().dest(2).payload(String::from("Hello"));
    assert!(res.contains(&expected_log));
    */
}









