pub mod Doc{
    pub struct Docs{
        style : String,
        text : String
    } 

    pub struct DocsMemento{
        style : String,
        text : String
    }

    impl Docs{
        pub fn new(style : String, text : String) -> Docs{
            Docs{
                style,
                text
            }

        }

        pub fn Save_State(&self) -> DocsMemento {
            DocsMemento { 
                style: String::from(&self.style),
                 text: String::from(&self.text)
            }
        }

        pub fn Restore_State(&mut self, DocsRest : DocsMemento ) -> &mut Self {
            self.style = DocsRest.style;
            self.text = DocsRest.text;
            self
        }

        pub fn set_style(&mut self, changeStyle : String) -> &mut Self {
            self.style = changeStyle;
            self
        }

        pub fn set_text(&mut self, changeText : String) -> &mut Self {
            self.text = changeText;
            self
        }

        pub fn get_style(&mut self) -> &String {
            &self.style
        }

        pub fn get_text(&mut self) -> &String {
            &self.text  
        }

    }



}