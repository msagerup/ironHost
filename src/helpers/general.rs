use crate::models::general::llm::Message;

// Extend AI function to encourage correct behavior.
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str: &str = ai_func(func_input);
    // Extend the string to provide context to the AI what to do with it, encourage only printing intended result.
    let msg: String = format!("FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the result of functions.
    NOTHING ELSE!. No commentary. Here is the input to the function: {}.
    Print out what the function will return.",
    ai_function_str, func_input );

    // Return message
    Message {
        role: "system".to_string(),
        content:msg
    }
 }


 #[cfg(test)]
 mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
       let extended_instruction: Message =  extend_ai_function(convert_user_input_to_goal, "dummy variable");
       assert_eq!(extended_instruction.role, "system".to_string());
    }
 }