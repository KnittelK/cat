pub trait Operation<'a>
{
    fn handle(&self, contents: String) -> String;

    fn execute(&mut self, contents: String) -> String;

    fn next(&mut self) -> &mut Option<Box<dyn Operation<'a> + 'a>>;
}