struct DPL {
    d: Option<String>,
    p: Option<String>,
    l: Option<String>,        
}




fn main() {
    println!("Hello, world!");

    let dpl_vec = vec!(
        DPL {d: None, p: Some("100".to_string()), l: None},
        DPL {d: Some("300".to_string()), p: None, l: None}, 
    );

    let statement = build_statement(&dpl_vec);

    println!("Statement generated = {}", statement);


}


fn build_statement (dpl_vec: &Vec<DPL>) -> String {

    // TODO look into using a string buffer equiv in Rust
    let mut statement = String::new();
    let mut first = true;

    for dpl in dpl_vec {

        if !first {
            // Put an OR at the beginning
            statement.push_str(" OR ");
        } else {
            first = false;
        }

        let d_term = match &dpl.d {
            Some(d) =>  format!("(d LIKE '{}')", d),
            None =>     "(d LIKE '%' OR d IS NULL)".to_string(),
        };
        let p_term = match &dpl.p {
            Some(p) =>  format!("(p LIKE '{}')", p),
            None =>     "(p LIKE '%' OR p IS NULL)".to_string(),
        };
        let l_term = match &dpl.l {
            Some(l) =>  format!("(l LIKE '{}')", l),
            None =>     "(l LIKE '%' OR l IS NULL)".to_string(),
        };
        statement.push_str(&*format!("( {} AND {} AND {} )", d_term, p_term, l_term));
    }

    statement
}







// select d,p,l,msg from dpl where 
// ((d LIKE '%' OR d IS NULL) AND (p LIKE'%' OR p IS NULL) AND (l LIKE '%' OR l IS NULL))


// select d,p,l,msg from dpl where 
// ((d LIKE '100' ) AND (p LIKE'%' OR p IS NULL) AND (l LIKE '%' OR l IS NULL)) 
// OR ((d LIKE '300') AND (p LIKE'%' OR p IS NULL) AND (l LIKE '%' OR l IS NULL))
