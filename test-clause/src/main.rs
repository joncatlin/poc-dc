fn main() {

    let clause = build_sql_where_clause_1 ("100".to_string(), "".to_string(), "20".to_string(), 42, vec!(1,2,3,4,5,6));
    println!("Clause: {}", clause);
}


fn build_sql_where_clause_1 (developer: String, project: String, lender: String,
    category_id: i32, correspondence_ids: Vec<i32>) -> String {

    let mut clause = String::new();

    clause.push_str(&*format!(" WHERE cp.developer = '{}' ", developer));
    if project != "".to_string() {
        clause.push_str(&*format!(" AND cp.project = '{}' ", project));
    }

    if lender != "".to_string() {
        clause.push_str(&*format!(" AND cp.lender = '{}' ", lender));
    }

    clause.push_str(&*format!(" AND cm.category_id = {} ", category_id));

    if correspondence_ids.len() != 0 {

        let mut in_clause = correspondence_ids.into_iter().map(|x| format!("{},", x)).collect::<String>();
        in_clause.truncate(in_clause.len()-1);
        clause.push_str(&*format!(" AND cm.correspondence_id IN ({}) ", in_clause.to_string()));
    }

    clause
}
