use quote::quote;
use syn::DeriveInput;

fn to_sql(ty: String) -> String {
    let mut is_option = false;
    let ty = match ty.as_str() {
        "bool" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => {
            "INTEGER".to_string()
        }
        "String" | "&str" => "TEXT".to_string(),
        "f32" | "f64" => "REAL".to_string(),
        outer if ty.starts_with("Option") => {
            is_option = true;
            let outer: String = outer.chars().filter(|c| !c.is_whitespace()).collect();
            let lenght = outer.len();
            let inner: String = outer.chars().skip(6).take(lenght - 8).collect();
            to_sql(inner)
        }
        other => panic!("`{other}` is unable to parse as SQL."),
    };
    if !is_option {
        return format!("{ty} NOT NULL");
    }
    ty
}

pub fn impl_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse2(input.into()).unwrap();
    let id = ast.ident;
    let columns: String = match ast.data {
        syn::Data::Struct(data) => data
            .fields
            .into_iter()
            .filter_map(|field| field.ident.zip(Some(field.ty)))
            .map(|(id, ty)| {
                let id = id.to_string();
                let ty = to_sql(quote! { #ty }.to_string());
                format!("{id} {ty}")
            })
            .collect::<Vec<String>>()
            .join(",\n"),
        _ => panic!("Only Structs are supported by Table."),
    };

    let schema = format!(
        "CREATE TABLE IF NOT EXISTS {} (\n{}\n);",
        id.to_string(),
        columns
    );

    quote! {
        impl Table for #id {
            fn schema() -> &'static str {
                #schema
            }
        }
    }
    .into()
}
