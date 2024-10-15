// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

const TAX_PERCENTAGE: f64 = 0.30;
const OWNER_PERCENTAGE: f64 = 0.55;
const PROFIT_PERCENTAGE: f64 = 0.05;
const OPERATION_PERCENTAGE: f64 = 0.10;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_divide_income(move |string| {
        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tax: f64 = num * TAX_PERCENTAGE;
        let owner: f64 = num * OWNER_PERCENTAGE;
        let profit: f64 = num * PROFIT_PERCENTAGE;
        let oper: f64 = num * OPERATION_PERCENTAGE;
        let result = format!("Taxes: {tax:.2}\nOwner: {owner:.2}\nProfit: {profit:.2}\nOpEx: {oper:.2}");
        ui.set_results(result.into());
    });

    ui.run();

    Ok(())
}
