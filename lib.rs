\
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::atomic::{AtomicUsize, Ordering};

// Simple global rate counter (process-level, reset handled externally)
static REQ_COUNTER: AtomicUsize = AtomicUsize::new(0);

unsafe fn cstr_to_string(raw: *const c_char) -> String {
    if raw.is_null() {
        return String::new();
    }
    CStr::from_ptr(raw).to_string_lossy().into_owned()
}

fn string_to_cstring(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn handle_input(raw_input: *const c_char) -> *mut c_char {
    let input = cstr_to_string(raw_input).to_lowercase();

    // Increment rate counter
    let cnt = REQ_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    if cnt > 50 {
        return string_to_cstring("⚠️ Rate limit exceeded — please try later.".to_string());
    }

    // Basic logging (returns in reply for demo)
    let mut log = format!("(log:{}) ", cnt);

    // Greeting
    if input.contains("hello") || input.contains("hi") {
        let resp = format!("{}👋 Hi! I am your UOMI Helper Agent. How can I help?", log);
        return string_to_cstring(resp);
    }

    // Roles
    if input.contains("role") {
        let resp = format!("{}🏅 Roles in UOMI: Validator, Delegator, Agent.", log);
        return string_to_cstring(resp);
    }

    // Token info
    if input.contains("token") {
        let resp = format!("{}💰 $UOMI is used for fees, staking, governance and rewards.", log);
        return string_to_cstring(resp);
    }

    // Validators
    if input.contains("validator") {
        let resp = format!("{}🔐 Validators secure the network and validate blocks.", log);
        return string_to_cstring(resp);
    }

    // Send simulation: expected forms: "send 5", "send 5 tokens"
    if input.starts_with("send ") {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(amount) = parts[1].parse::<u64>() {
                if amount > 100 {
                    let resp = format!("{}⚠️ Please confirm large transfer: send {} tokens? (reply: confirm send {})", log, amount, amount);
                    return string_to_cstring(resp);
                } else {
                    let resp = format!("{}✅ Transaction simulated: {} UOMI tokens sent!", log, amount);
                    return string_to_cstring(resp);
                }
            }
        }
        return string_to_cstring(format!("{}⚠️ Invalid send command. Use: send <amount>", log));
    }

    // Confirm send
    if input.starts_with("confirm send ") {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() >= 3 {
            if let Ok(amount) = parts[2].parse::<u64>() {
                let resp = format!("{}✅ Confirmed: {} UOMI tokens simulated as sent.", log, amount);
                return string_to_cstring(resp);
            }
        }
        return string_to_cstring(format!("{}⚠️ Invalid confirm format. Use: confirm send <amount>", log));
    }

    // Default fallback
    let resp = format!("{}🤔 I don't understand yet. Try: hello, role, token, validator, send <amount>.", log);
    string_to_cstring(resp)
}
