
pub mod History{

    use crate::MementoPattern::Doc::{Docs, DocsMemento};

    pub struct HistorySave{
        HistoryCollect : Vec<DocsMemento>
    } 

    impl HistorySave {
        
        pub fn new() -> HistorySave{
            HistorySave {
                HistoryCollect: Vec::new() 
            }
        }

        pub fn Save (&mut self, Docs_Save : DocsMemento) -> &mut Self {
            self.HistoryCollect.push(Docs_Save);
            self
        }

        pub fn Pop (&mut self) -> Option<DocsMemento> {
            self.HistoryCollect.pop()          
        }

    }
    
}