#![doc(html_logo_url = "https://github.com/dr-montasir/styledlog/raw/HEAD/logo.svg?sanitize=true")]
#![doc = r"<div align='center'><img src='https://github.com/dr-montasir/styledlog/raw/HEAD/logo.svg?sanitize=true' alt='logo' width='200' height='200' /></div><br>"]
#![doc = r"<div align='center'>Styledlog: A Rust crate combining versatile logging features with styled output.<br><br>
<img src='https://github.com/dr-montasir/styledlog/blob/main/styledlog.gif?raw=true' alt='StyledLog' width='100%' /></div>"]

pub mod colorful;
pub mod formatter;
pub mod level;

pub use colorful::*;
pub use formatter::*;
pub use level::{add_level, log_level, remove_level, Style};
