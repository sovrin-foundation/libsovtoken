use std::sync::{Mutex, Arc};

pub struct CallbackFailed;

pub fn map_async<C, F, R, E>(v: Vec<R>, cb_map: C, cb_finish: F)
    where
        C: Fn(R, Arc<Fn(Result<R, E>) + Send + Sync>) + Send + Sync + 'static,
        F: Fn(Result<Vec<R>, CallbackFailed>) + Send + Sync + 'static,
        R: Clone + Send + 'static
{

    let m: Arc<Mutex<Vec<R>>> = Default::default();
    let cb_finish_arc = Arc::new(cb_finish);
    let length = v.len();

    for value in v {
        let m_clone = m.clone();
        let cb_finish_arc_clone = cb_finish_arc.clone();
        let done = Arc::new(move |result| {
            if let Ok(mapped_value) = result {
                let mut guard = m_clone.lock().unwrap();
                guard.push(mapped_value);
                
                if guard.len() == length {
                    let mapped = (*guard).clone();
                    cb_finish_arc_clone(Ok(mapped));
                }
            }
            else {
                cb_finish_arc_clone(Err(CallbackFailed));
            };
        });

        cb_map(value, done);
    }

}


#[cfg(test)]
mod async_map_tests {
    use super::*;
    use std::thread;
    use std::time;


    #[test]
    fn test_async_add_1() {
        static mut B : bool = false;

        fn add_1<C>(v: Vec<u32>, cb: C)
            where C: Fn(Vec<u32>) + Send + Sync + 'static
        {
            map_async(v, move |value, f| {
                thread::spawn(move || f(value + 1));
            }, cb);
        }

        add_1(vec![1, 2, 3], move |v| {
            unsafe {B = true}
            assert_eq!(v, vec![2, 3, 4]);
        });
        thread::sleep(time::Duration::from_millis(500));
        assert!(unsafe { B });
    }
}
