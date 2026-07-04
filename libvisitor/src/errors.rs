use anyhow::Error;

#[derive(Debug)]
pub enum VError
{
    FILE_READ_ERROR{ e: Error },
}