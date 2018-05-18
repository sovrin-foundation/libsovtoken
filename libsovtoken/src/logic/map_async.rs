use std::thread;
use std::time;
use std::sync::{Mutex, Arc};
use std::cmp::Ordering;

//Box<Fn(I::Item, Box<Fn(R) + Send>)>
//fn map_async<I, C, F, R>(mut it: I, length: usize, cb: Box<C>, finished: Box<F>)
//    where I: Iterator,
//        C: FnMut(I::Item, Box<Fn(R) + Send>) + Send + 'static,
//        F: FnMut(Vec<R>) + Send + 'static,
//        R: Send + 'static
//{
//    let mapped: Arc<Mutex<Vec<R>>> = Default::default();
//
//    let idx = 1;
//    let v = it.next().unwrap();
//    let cloned_map = mapped.clone();
//    cb(v, Box::new(move |value: R| {
//            let mut guard = cloned_map.lock().unwrap();
//            guard.insert(idx, value);
//            guard.remove(idx + 1);
//
//            if guard.len() == length {
//                finished(*guard);
//            }
//    }));
//}
//
//fn async_add_number(numbers: Vec<u32>, add: u32, cb: Box<Fn(Vec<u32>)>) {
//    map_async(numbers.into_iter(), numbers.len(), Box::new(move |number: u32, done: Box<Fn(u32) + Send + 'static>| {
//        thread::spawn(|| {
//            let sum = number + add;
//            done(sum);
//        });
//    }), cb);
//}


//#[test]
//fn map_async_add_number() {
//    let numbers = vec![1, 2, 3, 4, 5];
//    let expected = vec![3, 4, 5, 6, 7];
//    async_add_number(numbers, 2, Box::new(|mapped| assert_eq!(mapped, numbers)));
//    thread::sleep(time::Duration::new(1));
//}

//#[test]
//fn map_async_add_number() {
//    let numbers = vec![1];
//    let expected = vec![3];
//    async_add_number(numbers, 2, Box::new(move|mapped: Vec<u32>| assert_eq!(mapped, numbers)));
//    thread::sleep(time::Duration::new(1, 0));
//}
//

//static mut B : bool = false;
//type MutFunc<X> = Box<FnMut(X) + Send>;
//type ArcMutFunc<X> = Arc<MutFunc<X>>;
//
//
//fn b<R>(mut cb: MutFunc<ArcMutFunc<R>>) {
//    let bcb = Box::new(|bool| unsafe { B = bool });
//    let mut arcedcb = Arc::new(bcb);
//    cb(arcedcb);
//}
//
//
//fn a(mut cb: Box<FnMut() + Send + Sync>) {
//    let mut arcedcb = Arc::new(cb);
//    let acb = Box::new(move |f| {
//        let mut a2 = arcedcb.clone();
//        thread::spawn(move || {
//            Arc::get_mut(&mut a2).unwrap()();
//            f(true);
//        } );
//    });
//    b(acb);
//}
//
//#[test]
//fn test_a() {
//    let mut vec = vec![1, 2, 3];
//    let vec2 = vec.clone();
//    a(Box::new(move || {
//        vec.push(5);
//    }));
//    assert_eq!(vec2, vec![1, 2, 3, 5]);
//    assert!(unsafe { B });
//}
//

//
//type MutFunc<X> = Box<FnMut(X) + Send + Sync>;
//type ArcMutFunc<X> = Arc<MutFunc<X>>;
//type CallBackMapper<X> = MutFunc<MutFunc<X>>;
//
//
//fn b<R>(length: usize, mut cb_mapper: CallBackMapper<R>, mut cb_finished: ArcMutFunc<Vec<R>>)
//    where R: Send + 'static
//{
//    let mapped_values: Arc<Mutex<Vec<R>>> = Default::default();
//
//    let mapped_values_clone = mapped_values.clone();
//    let mut cb_finished_clone = cb_finished.clone();
//    let idx = 0;
//
//    cb_mapper(Box::new(move |value: R| {
//        let mut guard = mapped_values_clone.lock().unwrap();
//        guard.insert(idx, value);
//        guard.remove(idx + 1);
//
//        if guard.len() == length {
//            let finished = Arc::get_mut(&mut cb_finished_clone).unwrap();
//            finished(*guard);
//        }
//    }));
//}
//
//
//fn a(mut finished_cb: MutFunc<Vec<u32>>) {
//    let mut arc_finished_cb = Arc::new(finished_cb);
//    let cb = Box::new(move |mut done: MutFunc<u32> | {
//          done(2);
////        thread::spawn(move | | done(2) );
//    });
//    b(1,cb, arc_finished_cb);
//}
//
//#[test]
//fn test_a() {
//    let mut vec = vec![1, 2, 3];
//    let vec2 = vec.clone();
//    a(Box::new(move |mut v| {
//        vec.append(&mut v);
//    }));
//    assert_eq!(vec2, vec![1, 2, 3, 2]);
//}

/*
    WORKING
*/
//static mut B : bool = false;
//type MutFunc<X> = Box<FnMut(X) + Send + Sync>;
//type ArcMutFunc<X> = Arc<MutFunc<X>>;
//
//
//fn b(mut cb: MutFunc<Box<FnMut(bool) + Send + Sync>>) {
//    cb(Box::new(|bool| unsafe { B = bool }));
//}
//
//fn a(mut cb: Box<FnMut() + Send + Sync>) {
//    let mut cb_arc = Arc::new(cb);
//
//    b(Box::new(move |mut f| {
////        let mut cb_arc_cloned = cb_arc.clone();
//        f(true);
//        let finished = Arc::get_mut(&mut cb_arc).unwrap();
//        finished();
//    }))
//}
//
//#[test]
//fn test_a() {
//    let mut vec = vec![1, 2, 3];
//    a(Box::new(move || {
//        vec.push(5);
//        assert_eq!(vec, vec![1, 2, 3, 5]);
//        assert!(unsafe { B });
//    }));
//    thread::sleep(time::Duration::from_millis(500));
//}

/*
    WORKING
*/
//static mut B : bool = false;
//
//fn b<C, F>(mut v: Vec<u32>, mut cb_map: C, cb_finish: F)
//    where C: Fn(Arc<Fn(bool) + Send + Sync>) + Send + Sync + 'static,
//          F: Fn(Vec<u32>) + Send + Sync + 'static
//{
//    v.push(5);
//    let v_arc = Arc::new(v);
//    let done = Arc::new(move|bool| {
//        unsafe {B = bool }
//        cb_finish(v_arc.to_vec());
//    });
//    cb_map(done);
//}
//
//fn a<C>(v: Vec<u32>, cb: C)
//    where C: Fn(Vec<u32>) + Send + Sync + 'static
//{
//    b(v, move |f| {
//        thread::spawn(move || f(true));
//    }, cb);
//}
//
//#[test]
//fn test_a() {
//    a(vec![1, 2, 3], move |v| {
//        assert_eq!(v, vec![1, 2, 3, 5]);
//        assert!(unsafe { B });
//    });
//    thread::sleep(time::Duration::from_millis(500));

static mut B : bool = false;

fn b<C, F>(v: Vec<u32>, cb_map: C, cb_finish: F)
    where C: Fn(u32, Arc<Fn(u32) + Send + Sync>) + Send + Sync + 'static,
        F: Fn(Vec<u32>) + Send + Sync + 'static
{

    let m: Arc<Mutex<Vec<u32>>> = Default::default();
    let cb_finish_arc = Arc::new(cb_finish);

    for value in v {
        let m_clone = m.clone();
        let cb_finish_arc_clone = cb_finish_arc.clone();
        let done = Arc::new(move|num| {

            let mut guard = m_clone.lock().unwrap();
            guard.push(num);

            if guard.len() == 3 {
                unsafe {B = true }
                let mapped = (*guard).clone();
                cb_finish_arc_clone(mapped);
            }
        });

        cb_map(value, done);
    }

}

fn a<C>(v: Vec<u32>, cb: C)
    where C: Fn(Vec<u32>) + Send + Sync + 'static
    {
    b(v, move |value, f| {
        thread::spawn(move || f(value + 1));
    }, cb);
}

#[test]
fn test_a() {
    a(vec![1, 2, 3], move |v| {
        assert_eq!(v, vec![2, 3, 4]);
        assert!(unsafe { B });
    });
    thread::sleep(time::Duration::from_millis(500));
}