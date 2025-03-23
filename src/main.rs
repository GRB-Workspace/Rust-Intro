mod control_flows;
mod custom_data_types;
mod data_types;
mod functions;
mod ownership;
mod references_and_borrowing;
mod variables;

/**
 * @author: Gayanuka Bulegoda
 * @github: https://github.com/gayanukabulegoda
 * @portfolio: https://grbulegoda.me
 * -------------------------------------------------------------------
 * @project: Rust Intro
 * @repository: https://github.com/GRB-Workspace/Rust-Intro.git
 * @since: 21-03-2025 10.00 AM
 * -------------------------------------------------------------------
 * @module: main.rs (Main Module)
 * -------------------------------------------------------------------
 */

fn main() {
    let module = "variables";

    match module {
        "variables" => variables::variables_intro(),
        "data_types" => data_types::data_types_intro(),
        "custom_data_types" => custom_data_types::custom_data_types_intro(),
        "functions" => functions::functions_intro(),
        "control_flows" => control_flows::control_flows_intro(),
        "ownership" => ownership::rust_ownership_intro(),
        "references_and_borrowing" => references_and_borrowing::references_and_borrowing_intro(),
        _ => println!("Invalid module name"),
    }

    // variables::variables_intro();
    // data_types::data_types_intro();
    // custom_data_types::custom_data_types_intro();
    // functions::functions_intro()
    // control_flows::control_flows_intro();
    // ownership::rust_ownership_intro();
    // references_and_borrowing::references_and_borrowing_intro();
}
