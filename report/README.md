# report

Report is a type that allows for comprehensive error reporting to the terminal. It aims to replace `Result<T, E>` in large applications who need multi-error handling, additional metadata, backtracking to error cause.

The `Report` type contains three modes:
- `Main`: default mode when a report is created,
- `Same`: does not increase the logging level
- `Name<"section">`: increases the logging level

A report can be derived to be passed on to another function with the `.to()` method, that automatically adjusts the logging level.

When a named report is dropped, the logging level is reduced and the caller gets control back.

`Report` contains a `.issue(impl Into<Issue>) -> Option<!>` method that takes any object that can be converted into an issue and sends it to the terminal before returning `None`, and a `.eat(Result<Type, Issue>) -> Option<Type>` that reduces a result possibly containing an issue to an option.

See [the documentation](https://docs.rs/libutils-report) for more information.