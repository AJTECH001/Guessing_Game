use leptos::*;
use rand::Rng;
use std::cmp::Ordering;

#[component]
fn App() -> impl IntoView {
    let (secret_number, set_secret_number) = create_signal(rand::thread_rng().gen_range(1..=100));
    let (guess, set_guess) = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());
    let (game_over, set_game_over) = create_signal(false);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let guess_value = guess.get().trim().parse::<u32>();
        match guess_value {
            Ok(num) => {
                match num.cmp(&secret_number.get()) {
                    Ordering::Less => {
                        set_message.set("Too small!".to_string());
                        set_guess.set(String::new());
                    }
                    Ordering::Greater => {
                        set_message.set("Too big!".to_string());
                        set_guess.set(String::new());
                    }
                    Ordering::Equal => {
                        set_message.set("You win!".to_string());
                        set_game_over.set(true);
                    }
                }
            }
            Err(_) => {
                set_message.set("Please enter a valid number!".to_string());
                set_guess.set(String::new());
            }
        }
    };

    let reset_game = move |_| {
        set_secret_number.set(rand::thread_rng().gen_range(1..=100));
        set_message.set(String::new());
        set_guess.set(String::new());
        set_game_over.set(false);
    };

    view! {
        <div class="container">
            <h1>"Guess the Number!"</h1>
            <p>"The secret number is: " {move || secret_number.get()}</p> // Remove in production
            <form on:submit=handle_submit>
                <input
                    type="text"
                    placeholder="Enter your guess"
                    value=guess
                    on:input=move |ev| set_guess.set(event_target_value(&ev))
                    disabled=move || game_over.get()
                />
                <button type="submit" disabled=move || game_over.get()>"Submit Guess"</button>
            </form>
            <p class=move || if message.get().contains("win") { "win" } else { "error" }>
                {move || message.get()}
            </p>
            <button on:click=reset_game disabled=move || !game_over.get()>"Play Again"</button>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> });
}