use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    thread,
};

pub fn test_threads() {
    let stdout = Arc::new(Mutex::new(std::io::stdout()));
    {
        let mut guard = stdout.lock().unwrap();
        writeln!(&mut guard, "[Main] start").unwrap();
    }

    // thread::spawn(move || mythreads("B", Arc::clone(&stdout)));
    // Do not this way.                                ^^ variable moved due to use in closure

    // creates another pointer to the same allocation
    let cloned_stdout1 = Arc::clone(&stdout);
    let t1 = thread::spawn(move || mythreads("[T1] A", cloned_stdout1));

    // creates another pointer to the same allocation
    let cloned_stdout2 = Arc::clone(&stdout);
    let t2 = thread::spawn(move || mythreads("[T2] B", cloned_stdout2));

    // wiat for threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    {
        let mut guard = stdout.lock().unwrap();
        let ref_mut_dereference = &mut guard;
        writeln!(*ref_mut_dereference, "[Main] done").unwrap();
    }
}

fn mythreads(arg: &str, stdout: Arc<Mutex<std::io::Stdout>>) {
    // Acquires a mutex, blocking the current thread until it is able to do so.
    let mut guard = stdout.lock().unwrap();
    writeln!(&mut *guard, "{}", arg).unwrap();
}
