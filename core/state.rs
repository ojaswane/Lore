// This file will print the output from shell to the ratatui 

fn output_shell(
    mut reader: Box<dyn Read + Send>, // the dyn keyword is to tell that it will be complied at realtime 
    output: Arc<Mutex<String>>,
) {

}