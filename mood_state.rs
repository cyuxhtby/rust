// This is an example of a simple Ink! smart contract for the Substrate blockchain framework.
// It allows the storage of an arbitrary string value referenced in this contract as `mood`.
use ink_lang::contract;

#[ink::contract]
// This attribute indicates that the following Rust module will be treated as an Ink! smart contract.
// Contracts must be fully enclosed within a module to specify the code to be compiled into Wasm bytecode.
mod mood_state {

    #[ink(storage)]
    // This struct will serve as the storage for our smart contract.
    pub struct MoodState {
        mood: String,
    }

    impl MoodState {
        #[ink(constructor)]
        // This function acts as the constructor for our contract and initializes mood with an empty string.
        // `Self` refers to the type being implemented, in this case `MoodState`.
        pub fn new() -> Self {
            Self {
                mood: String::new(),
            }
        }

        #[ink(message)]
        // This function allows callers to set the `mood` field of the contract.
        // The `&mut self` parameter indicates that this method can mutate the instance it's called on.
        pub fn set_mood(&mut self, mood: String) {
            // `self.mood` refers to the `mood` field of the current instance, similar to `this` in other languages.
            self.mood = mood;
        }

        #[ink(message)]
        // This function allows users to retreive the mood if one has been set.
        // `&self` means this function takes an immutable reference to the instance. 
        // It can read from the instance's data but cannot modify it.
        pub fn get_mood(&self) -> String {
             // Check if a mood has been set.
            if self.mood.is_empty() {
                "Mood has yet to be set".to_string()
            } else {
                // Otherwise, we return a clone of the `mood` field. 
                // Since we only have an immutable reference to the instance, we cannot transfer ownership of `mood` directly.
                // Therefore, we create a new `String` that's a copy of `mood` and return that.
                self.mood.clone()
            }
        }
    }
}
