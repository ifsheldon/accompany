macro_rules! bound {
    (with $patterned_identifier:pat = $expression:expr=>$bound_code:block)=>{
       {
           // patterned_identifier includes examples like tuple destruction, struct destruction and conventional identifier
           let $patterned_identifier = $expression;
           {$bound_code}
       }
    }
}


#[cfg(test)]
mod macro_tests {
    struct A {
        pub field: u8,
        pub field_u16: u16,
    }

    #[test]
    fn test_with_bound() {
        let a = A {
            field: 1,
            field_u16: 2,
        };
        let A { field: i, field_u16: j } = a;
        let simple_result = bound! {
            with i = 3 => {
                let m = i+1;
                m
            }
        };
        let destruct_result = bound! {
            with A { field: i, field_u16: _j } = a =>{
                let m = i+1;
                m
            }
        };
        let detuple_result = bound! {
            with (i,_j)=(4,3) =>{
                let m = i+1;
                m
            }
        };
        println!("{}", simple_result);
        println!("{}", destruct_result);
        println!("{}", detuple_result);
    }
}
