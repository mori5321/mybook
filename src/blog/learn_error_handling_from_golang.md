# Learning Error Handling From Golang.

## Four Patterns of Error Handling

- 1. Delegate to upstream.
- 2. Log(Provide feedback to User).
- 3. Retry
- 4. Cease Continuation and Trigger Panic/Exit


## Principles of Error Handling
### Don't delegate Error Handling to upstream as much as possible

The deeper the hierarchy, the more complex and difficult it becomes to follow the process. 
It's good that upstream just do "Logging Error" about Error Handlin. 
Handling other than logging should be done close to the process calling.


### Avoid defining Custom Error as much as possible.
Custom Error involves delegating error handling upstream. Handle errors as close to the process as possible.

### If your error message is good, you don't need stacktrace. 
Stacktrace shows its value only when error message is not useful. 

## Wrap & Unwrap
TBD

## Whether to Wrap
It depends on the context. Do not wrap an errror when doing so would expose implementation details. You should care about *Abstraction Violation*.

### Ex1. Parser
Imagine Parser which reads a complex data structure. If an error occurs, we wish to report the line and column number at whitch it occurred. It makes sense to expose the error produced by it.

### Ex2. Database Caller
Imagene Function which makes several calls to a database. It should not return an error which unwrap s to the result of one of those calls. If the database used by the function is an implementation detail, then exposes errors is a violation of abstraction. 

``` go:bad_example.go
// BAD: Abstraction Violation.
// If you wrap sql.ErrNoRows, your caller need to know sql.ErrNoRows to handle the error.
err := repo.LookupUser
if errors.Is(err, sql.ErrNoRows)
```

### Conclusion: Whether to wrap
- Whether you wrap or not, the error text will be the same. User gets the same information either way.
- The choice to wrap is about whether to give programs additional infomation so that they can make more informed decisions or to withhold that information to preserve an abstraction layer

## References
- [Golang Errors](https://earthly.dev/blog/golang-errors/)
- [Working with Errors in Go1.13](https://go.dev/blog/go1.13-errors)
- [Error Handling and Go](https://go.dev/blog/error-handling-and-go)
- [Go Error Handling Strategy / Goエラーハンドリング戦略](https://zenn.dev/nobonobo/articles/0b722c9c2b18d5#%E3%82%A8%E3%83%A9%E3%83%BC%E3%83%8F%E3%83%B3%E3%83%89%E3%83%AA%E3%83%B3%E3%82%B0%E3%81%AE%EF%BC%94%E7%A8%AE)
- [errors package Unwrap Is As](https://debimate.jp/2021/12/14/)
