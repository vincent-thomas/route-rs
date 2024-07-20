use route_http::request::Head;

use super::{Guard, GuardOutcome, GuardReason};

pub fn check_guards(guards: &[Box<dyn Guard>], parts: &Head) -> Option<GuardReason> {
  for guard in guards {
    match guard.check(parts) {
      GuardOutcome::Reason(reason) => return Some(reason),
      _ => continue,
    }
  }
  None
}
