use maud::{html, Markup, DOCTYPE};

pub fn helper_create_email_reset(token: &str, name: &str) -> Markup {
    html! {

        (DOCTYPE)

        html {

            head {
                title { "Reset Your Password" }

                meta charset="utf-8";
            }
        
            body {
                div style="display: flex; flex-dirction: column; align-items: center;" {
                
                    h2 { "Hello, " (name) }
                    
                    p { "A request has been received to change the password for your E-learning account."}
                    
                    p {"Reset: " }
                    
                    a href= { "http://127.0.0.1:8000/account/password/rest/" (token) } {"Reset your password"}
                    
                    p { "If you don't wish to reset your password, disregard this email and no action will be taken."}
                    
                    p { "Thank you,"}
                    p { "The E-learnig Team"}
                }
            }
        }
    }
}
