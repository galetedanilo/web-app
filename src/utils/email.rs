use maud::{html, Markup, DOCTYPE};

pub fn helper_activate_account_email(app_name: &str, domain_url: &str, name: &str, token: &str) -> Markup {

    html! {

        (DOCTYPE)

        html {
            
            head {
                title { "Activate your account"}

                meta charset="utf-8";
            }

            body {
                div sytle="display: flex; flex-direction: column; align-items: center;" {
                    h2 { "Welcome to " (app_name) }

                    p { "Hi " (name) }

                    p { "Thank you for signing up for our elearning platform." }

                    p { "To complete your registration click on the link below to activate your account and access our platform" }

                    a href = { (domain_url)"/account/activate/" (token) } { "Activate"}
                }

            }
        }
    }
}

pub fn helper_reset_password_email (token: &str, name: &str) -> Markup {
    html! {

        (DOCTYPE)

        html {

            head {
                title { "Reset Your Password" }

                meta charset="utf-8";
            }
        
            body {
                div style="display: flex; flex-direction: column; align-items: center;" {
                
                    h2 { "Hello, " (name) }
                    
                    p { "A request has been received to change the password for your E-learning account."}
                    
                    p {"Reset: " }
                    
                    a href = { "http://127.0.0.1:8000/account/password/rest/" (token) } {"Reset your password"}
                    
                    p { "If you don't wish to reset your password, disregard this email and no action will be taken."}
                    
                    p { "Thank you,"}
                    p { "The E-learnig Team"}
                }
            }
        }
    }
}
