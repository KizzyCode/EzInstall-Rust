/// Helpers to print context before and after a task is executed
#[macro_export]
macro_rules! run_verbose {
    ($cmd:expr => $result:ident, $before:expr, $after:expr) => ({
        $before;
        let $result = $cmd;
        $after;
        $result
    });
    ($cmd:expr, $before:expr, $after:expr) => ({
        $before;
        $cmd;
        $after;
    });
    ($cmd:expr, $before:expr) => ({
        $before;
        $cmd
    });
}