#![doc(html_logo_url = "https://raw.githubusercontent.com/dr-montasir/styledlog/main/logo.svg")]
#![doc = r"<div align='center'><img src='https://raw.githubusercontent.com/dr-montasir/styledlog/main/logo.svg' alt='logo' width='200' height='200' /></div><br>"]
#![doc = r"<div align='center'>Styledlog: A Rust crate combining versatile logging features with styled output.<br><br>
<img src='https://raw.githubusercontent.com/dr-montasir/styledlog/main/styledlog.gif' alt='StyledLog' width='100%' /></div>"]

pub mod colorful;
pub mod formatter;
pub mod level;

pub use colorful::*;
pub use formatter::*;
pub use level::{add_level, log_level, remove_level, Style};
