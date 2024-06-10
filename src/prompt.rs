pub fn prompt(schema: String) -> String {
    let final_prompt = format!(
        r#" Here is the schema for a database:

{schema}

Given this schema, can you output a SQL query to answer the following question? Only output the SQL query and nothing else.

Question:
"#,
        schema = schema
    );

    return final_prompt;
}
