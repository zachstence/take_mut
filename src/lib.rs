// TODO genericize
pub fn take_mut(v: & mut [MyStruct], i: usize) -> (Option<& mut MyStruct>, Vec<Option<&mut MyStruct>>) {
    let (before, after) = v.split_at_mut(i);
    let (taken, after) = after.split_at_mut(1);
    let taken = taken.get_mut(0);

    println!("Before {before:?}");
    println!("Take {taken:?}");
    println!("After {after:?}");

    let before_options = before.iter_mut().map(Some).collect::<Vec<Option<&mut MyStruct>>>();
    let mut after_options = after.iter_mut().map(Some).collect::<Vec<Option<&mut MyStruct>>>();
    
    let mut rest = before_options;
    rest.push(None);
    rest.append(&mut after_options);

    println!("Rest {rest:?}");

    (taken, rest)
}


//////


#[derive(Debug)]
pub struct MyStruct {
    x: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![
            MyStruct { x: 0 },
            MyStruct { x: 1 },
            MyStruct { x: 2 },
            MyStruct { x: 3 },
            MyStruct { x: 4 },
        ];

        let (taken, mut rest) = take_mut(&mut v, 2);

        // Should take correct item
        assert_eq!(taken.unwrap().x, 2);
        
        for (i, s) in rest.iter_mut().enumerate() {
            // Should have correct things in correct places
            if let Some(ref mut s) = s {
                assert_eq!(i as i32, s.x);

                // Should be able to mutate
                s.x = -(i as i32);
            } else {
                assert_eq!(i as i32, 2);
            }
        }

        for (i, s) in rest.iter().enumerate() {
            if let Some(s) = s {
                assert_eq!(-(i as i32), s.x);
            }
        }
    }
}
