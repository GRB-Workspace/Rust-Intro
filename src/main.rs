mod collections;
mod control_flows;
mod custom_data_types;
mod data_types;
mod enums;
mod example_struct;
mod functions;
mod garden;
mod hash_maps;
mod if_let_construct;
mod match_statement;
mod methods_for_structs;
mod ownership;
mod paths;
mod recoverable_errors;
mod references_and_borrowing;
mod rust_modules;
mod slices;
mod strings_and_utf8;
mod structs;
mod unrecoverable_errors;
mod use_keyword;
mod variables;
mod generics;
mod traits;
mod lifetimes;
mod reading_files;

/**
 * @author: Gayanuka Bulegoda
 * @github: https://github.com/gayanukabulegoda
 * @portfolio: https://grbulegoda.me
 * -------------------------------------------------------------------
 * @project: Rust Intro
 * @repository: https://github.com/GRB-Workspace/Rust-Intro.git
 * @since: 22-03-2025 10.00 AM
 * -------------------------------------------------------------------
 * @module: main.rs (Main Module)
 * -------------------------------------------------------------------
 */

fn main() {
    let module = "reading_files";

    match module {
        "variables" => variables::variables_intro(),
        "data_types" => data_types::data_types_intro(),
        "custom_data_types" => custom_data_types::custom_data_types_intro(),
        "functions" => functions::functions_intro(),
        "control_flows" => control_flows::control_flows_intro(),
        "ownership" => ownership::rust_ownership_intro(),
        "references_and_borrowing" => references_and_borrowing::references_and_borrowing_intro(),
        "slices" => slices::slices_intro(),
        "structs" => structs::structs_intro(),
        "example_struct" => example_struct::example_struct_intro(),
        "methods_for_structs" => methods_for_structs::methods_for_structs_intro(),
        "enums" => enums::enums_intro(),
        "match_statement" => match_statement::match_statement_intro(),
        "if_let_construct" => if_let_construct::if_let_construct(),
        "rust_modules" => rust_modules::modules_intro(),
        "paths" => paths::paths_intro(),
        "use_keyword" => use_keyword::use_keyword_intro(),
        "collections" => collections::collections_intro(),
        "strings_and_utf8" => strings_and_utf8::strings_and_utf8_intro(),
        "hash_maps" => hash_maps::hash_maps_intro(),
        "unrecoverable_errors" => unrecoverable_errors::unrecoverable_errors_intro(),
        "recoverable_errors" => recoverable_errors::recoverable_errors_intro(),
        "generics" => generics::generics_intro(),
        "traits" => traits::traits_intro(),
        "lifetimes" => lifetimes::lifetimes_intro(),
        "reading_files" => reading_files::reading_files_intro(),
        _ => println!("Invalid module name"),
    }
}
