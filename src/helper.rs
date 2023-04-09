use crate::if_std;
if_std! {
    use {
        std::time::{SystemTime, UNIX_EPOCH},
        time::format_description::well_known::Rfc3339
    };
}

pub(crate) fn now() -> i64 {
    #[cfg(feature = "std")]
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    #[cfg(not(feature = "std"))]
    0
}

#[allow(unused_variables)]
pub fn format_unix_timestamp(unix: i64) -> String {
    #[cfg(not(feature = "std"))]
    return String::from("");
    #[cfg(feature = "std")]
    time::OffsetDateTime::from_unix_timestamp(unix)
        .unwrap()
        .format(&Rfc3339)
        .unwrap()
}

macro_rules! instant_display_helper {
    ($self: ident, $ret: ident, $entry: expr) => {
        #[cfg(feature = "instant-display")]
        if $self.instant_display {
            match $self.join {
                true => $self.instant_display_helper(),
                false => ($self.display_fn)(),
            }
            return $ret;
        }
    };
    ($self: ident,e, $entry: expr) => {
        #[cfg(feature = "instant-display")]
        if $self.instant_display {
            match $self.join {
                true => $self.instant_display_helper(),
                false => ($self.display_fn)($entry.get_level(), $entry.as_string()),
            }
        }
    };
    ($self: ident, $ret: ident) => {
        #[cfg(feature = "instant-display")]
        if $self.instant_display {
            $self.instant_display_helper();
            return $ret;
        }
    };
    ($self: ident) => {
        #[cfg(feature = "instant-display")]
        if $self.instant_display {
            $self.instant_display_helper();
        }
    };
}
use alloc::string::String;
pub(crate) use instant_display_helper;
