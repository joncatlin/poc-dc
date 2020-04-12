struct DPL {
    d: String,
    p: String,
    l: String,
}


pub struct Hierarchy {
    pub developer: String,
    pub project: String,
    pub lender: String,
}


fn main() {
    println!("Hello, world!");
    
    let dpl_vec = vec!(
        Hierarchy {developer: "".to_string(), project: "100".to_string(), lender: "".to_string()},
        Hierarchy {developer: "300".to_string(), project: "".to_string(), lender: "".to_string()}, 
    );

    let statement = build_sql_statement(&dpl_vec);

    println!("Statement generated = {}", statement);


}


fn build_sql_statement (dpl_vec: &Vec<Hierarchy>) -> String {

    // TODO look into using a string buffer equiv in Rust
    let mut statement = String::new();
    let mut first = true;

    statement.push_str(" category_id = $1 AND ");

    for dpl in dpl_vec {

        if !first {
            // Put an OR at the beginning
            statement.push_str(" OR ");
        } else {
            first = false;
        }

        let d_term = match &dpl.developer[..] {
            "" => "(developer LIKE '%' OR developer IS NULL)".to_string(),
            d =>  format!("(developer LIKE '{}')", d),
        };
        let p_term = match &dpl.project[..] {
            "" => "(project LIKE '%' OR project IS NULL)".to_string(),
            p =>  format!("(project LIKE '{}')", p),
        };
        let l_term = match &dpl.lender[..] {
            "" => "(lender LIKE '%' OR lender IS NULL)".to_string(),
            l =>  format!("(lender LIKE '{}')", l),
        };
        statement.push_str(&*format!("( {} AND {} AND {} )", d_term, p_term, l_term));
    }

    statement
}


// fn build_statement (dpl_vec: &Vec<DPL>) -> String {

//     // TODO look into using a string buffer equiv in Rust
//     let mut statement = String::new();
//     let mut first = true;

//     for dpl in dpl_vec {

//         if !first {
//             // Put an OR at the beginning
//             statement.push_str(" OR ");
//         } else {
//             first = false;
//         }

//         let d_term = match &dpl.d {
//             Some(d) =>  format!("(d LIKE '{}')", d),
//             None =>     "(d LIKE '%' OR d IS NULL)".to_string(),
//         };
//         let p_term = match &dpl.p {
//             Some(p) =>  format!("(p LIKE '{}')", p),
//             None =>     "(p LIKE '%' OR p IS NULL)".to_string(),
//         };
//         let l_term = match &dpl.l {
//             Some(l) =>  format!("(l LIKE '{}')", l),
//             None =>     "(l LIKE '%' OR l IS NULL)".to_string(),
//         };
//         statement.push_str(&*format!("( {} AND {} AND {} )", d_term, p_term, l_term));
//     }

//     statement
// }







// select d,p,l,msg from dpl where 
// ((d LIKE '%' OR d IS NULL) AND (p LIKE'%' OR p IS NULL) AND (l LIKE '%' OR l IS NULL))


// select d,p,l,msg from dpl where 
// ((d LIKE '100' ) AND (p LIKE'%' OR p IS NULL) AND (l LIKE '%' OR l IS NULL)) 
// OR ((d LIKE '300') AND (p LIKE'%' OR p IS NULL) AND (l LIKE '%' OR l IS NULL))
