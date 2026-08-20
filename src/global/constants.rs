pub const APP_NAME: &str = "rools";
pub const APP_AUTHOR: &str = "sJ3x4AiGYbt";
pub const APP_VERSION: &str = "0.1.0";
pub const BANNER_ROOLS: &str = r#"

 ,ggggggggggg,      _,gggggg,_        _,gggggg,_           ,gggg,        ,gg,   
dP"""88""""""Y8,  ,d8P""d8P"Y8b,    ,d8P""d8P"Y8b,        d8" "8I       i8""8i  
Yb,  88      `8b ,d8'   Y8   "8b,dP,d8'   Y8   "8b,dP     88  ,dP       `8,,8'  
 `"  88      ,8P d8'    `Ybaaad88P'd8'    `Ybaaad88P'  8888888P"         `88'   
     88aaaad8P"  8P       `""""Y8  8P       `""""Y8       88             dP"8,  
     88""""Yb,   8b            d8  8b            d8       88            dP' `8a 
     88     "8b  Y8,          ,8P  Y8,          ,8P  ,aa,_88           dP'   `Yb
     88      `8i `Y8,        ,8P'  `Y8,        ,8P' dP" "88P       _ ,dP'     I8
     88       Yb, `Y8b,,__,,d8P'    `Y8b,,__,,d8P'  Yb,_,d88b,,_   "888,,____,dP
     88        Y8   `"Y8888P"'        `"Y8888P"'     "Y8P"  "Y88888a8P"Y88888P" 
"#;

/* permissions.rs */
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub const ERROR_NEED_ROOT: &str = "[error]   root privileges required, run again with: sudo";
#[cfg(target_os = "windows")]
pub const ERROR_NEED_ADMIN_WINDOWS: &str = "[error]   administrator privileges required, run again from an elevated terminal (Run as administrator):";

/* shutdown.rs */
pub const ERROR_CTRLC: &str = "[error]   setting Ctrl-C handler";

/* writter.rs */
pub const ERROR_FORMATTER_FILE: &str = "[error]   failed to create output file:";

/* export.rs */
pub const EXPORT_SUCCESS: &str = "[success] results exported to:";
pub const ZIP_SUCCESS: &str = "[success] compressed to:";
pub const WARN_FILE_REMOVAL: &str = "[warning] could not remove original file";
