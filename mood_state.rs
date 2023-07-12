use ink_lang::contract;

#[ink::contract]
// Ink! contracts must be enclosed within a module to specify the code to be compiled into Wasm bytecode
mod mood_state {

    #[ink(storage)]
    pub struct MoodState {
        mood: String,
    }

    impl MoodState {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                mood: String::new(),
            }
        }

        #[ink(message)]
        pub fn set_mood(&mut self, mood: String) {
            self.mood = mood;
        }

        #[ink(message)]
        pub fn get_mood(&self) -> String {
            self.mood.clone()
        }
    }
}