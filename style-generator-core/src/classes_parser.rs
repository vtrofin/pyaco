use anyhow::Result;
use cssparser::{
    AtRuleParser, AtRuleType, BasicParseErrorKind, CowRcStr, ParseError, Parser, ParserState,
    QualifiedRuleParser, Token,
};

pub struct ClassesParser;

impl<'i> QualifiedRuleParser<'i> for ClassesParser {
    type Prelude = Vec<String>;
    type QualifiedRule = Vec<String>;
    type Error = ();

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let mut ret = Vec::new();

        while !input.is_exhausted() {
            match input.next() {
                Ok(Token::Delim('.')) => {
                    if let Ok(Token::Ident(ident)) = input.next() {
                        ret.push(ident.to_string());
                    } else {
                        return Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid));
                    }
                }
                Ok(_) => (),
                Err(_) => return Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid)),
            }
        }

        Ok(ret)
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        // Consume the block input
        while input.next().is_ok() {
            continue;
        }

        Ok(prelude)
    }
}

impl<'i> AtRuleParser<'i> for ClassesParser {
    type PreludeNoBlock = ();
    type PreludeBlock = ();
    type AtRule = Vec<String>;
    type Error = ();

    #[allow(clippy::type_complexity)]
    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<AtRuleType<Self::PreludeNoBlock, Self::PreludeBlock>, ParseError<'i, Self::Error>>
    {
        let ret = match name.to_string().as_str() {
            "media" => Ok(AtRuleType::WithBlock(())),
            _ => Ok(AtRuleType::WithoutBlock(())),
        };

        // Consume the rest of the input
        while input.next().is_ok() {
            continue;
        }

        ret
    }

    fn parse_block<'t>(
        &mut self,
        mut _prelude: Self::PreludeBlock,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, ()>> {
        let mut ret = Vec::new();

        while !input.is_exhausted() {
            match input.next() {
                Ok(Token::Delim('.')) => {
                    if let Ok(Token::Ident(ident)) = input.next() {
                        ret.push(ident.to_string());
                    } else {
                        return Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid));
                    }
                }
                Ok(_) => (),
                Err(_) => return Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid)),
            }
        }

        Ok(ret)
    }
}
