// The main goal here is to handle shell output in a background thread,
// so blocking reads from the shell do not freeze the main thread.
// This keeps the main thread free to handle UI rendering, user input,
// and other application logic.
