extern crate core_affinity;

use std::thread;

fn main() {
    // Retrieve the IDs of all active CPU cores.
    let core_ids = core_affinity::get_core_ids().unwrap();

    // Create a thread for each active CPU core.
    let handles = core_ids
        .into_iter()
        .map(|id| {
            thread::spawn(move || {
                // Pin this thread to a single CPU core.
                core_affinity::set_for_current(id);
                // Do more work after this.
                println!("id is: {:?}", id);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}
