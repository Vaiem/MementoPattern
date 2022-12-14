mod MementoPattern;
mod historySave;
use std::collections;
use crate::historySave::History::HistorySave;

fn main() {
    let mut myDoc = MementoPattern::Doc::Docs::new("awada".to_string(), "text".to_string());
    let mut HistorySave = HistorySave::new();

    myDoc
        .set_text("newSetStyle".to_string())
        .set_style("newSetText".to_string());
    
    print!("ferst style :{:?} ", myDoc.get_style());
    println!("ferst Text : {:?} ", myDoc.get_text());

    HistorySave.Save(myDoc.Save_State());

    myDoc
        .set_text("TwoSetStyle".to_string())
        .set_style("TwoSetText".to_string());

    print!("Two style :{:?} ", myDoc.get_style());
    println!("Two Text : {:?} ", myDoc.get_text());

    match HistorySave.Pop() {
        Some(res) => {
            myDoc.Restore_State(res);
        },
        None => println!("History is empty"),
    }

    print!("Reset style :{:?} ", myDoc.get_style());
    println!("Reset Text : {:?} ", myDoc.get_text());

}
