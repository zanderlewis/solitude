/* Comments */
pub const ML_COMMENT: &str = "...";
pub const SL_COMMENT: &str = ".";

/* Variables */
pub const VAR_DECLARE: &str = "var ";
pub const VAR_DELETE: &str = "-";
pub const VAR_GET: char = '$';

/* Functions */
pub const FUNC_BEGIN: &str = "func ";
pub const FUNC_END: &str = "cnuf";
pub const FUNC_CALL: &str = "call ";

/* Conditionals */
pub const IF_BEGIN: &str = "if ";
pub const IF_END: &str = "fi";

/* Threading */
pub const THREAD_OUTSIDE_BEGIN: &str = "!!";
pub const THREAD_INSIDE_BEGIN: &str = "{";
pub const THREAD_INSIDE_END: &str = "}";
pub const THREAD_OUTSIDE_END: &str = "??";

/* Input */
pub const INPUT_VAR: &str = "input ";
pub const INPUT_VAR_SPLIT: &str = "->";