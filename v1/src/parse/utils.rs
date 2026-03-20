use std::iter::Peekable;
use std::str::Chars;

pub fn is_end_term(c: Option<&char>) -> bool
{
	return matches!(c, Some('+') | Some('-') | Some('=') | None);
}

pub fn is_operation(c: Option<&char>) -> bool
{
	return matches!(c, Some('+') | Some('-') | Some('*') | Some('/') | Some('='));
}

pub fn is_end(c: Option<&char>) -> bool
{
	return c == Some(&'=') || c == None;
}

pub fn skip_space(chars: &mut Peekable<Chars<'_>>) -> ()
{
	while let Some(c) = chars.peek() {
		if *c != ' ' { break ; }
		chars.next();
	}
	return ;
}
