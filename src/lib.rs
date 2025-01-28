#![doc(html_logo_url = "https://github.com/dr-montasir/styledlog/raw/HEAD/logo.svg")]
#![doc = r"<div align='center'><img src='https://github.com/dr-montasir/styledlog/raw/HEAD/logo.svg' alt='logo' width='200' height='200' /></div><br>"]
#![doc = r"<div align='center'>Styledlog: A Rust crate combining versatile logging features with styled output.<br><br>
<img src='https://github.com/dr-montasir/styledlog/raw/main/styledlog.gif' alt='StyledLog' width='100%' height='auto' /></div>"]

pub mod colorful;
pub mod formatter;
pub mod level;

pub use colorful::*;
pub use formatter::*;
pub use level::{add_level, log_level, remove_level, Style};
