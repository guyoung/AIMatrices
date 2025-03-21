use crate::ctx::{Context, MutableContext};
use crate::dbs::{Iterator, Options, Statement};
use crate::doc::CursorDoc;
use crate::err::Error;
use crate::sql::{Cond, Output, Timeout, Value, Values};
use derive::Store;
use reblessive::tree::Stk;
use revision::revisioned;
use serde::{Deserialize, Serialize};
use std::fmt;

#[revisioned(revision = 2)]
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Store, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub struct DeleteStatement {
    #[revision(start = 2)]
    pub only: bool,
    pub what: Values,
    pub cond: Option<Cond>,
    pub output: Option<Output>,
    pub timeout: Option<Timeout>,
    pub parallel: bool,
}

impl DeleteStatement {
    /// Check if we require a writeable transaction
    pub(crate) fn writeable(&self) -> bool {
        true
    }
    /// Process this type returning a computed simple Value
    pub(crate) async fn compute(
        &self,
        stk: &mut Stk,
        ctx: &Context,
        opt: &Options,
        doc: Option<&CursorDoc>,
    ) -> Result<Value, Error> {
        // Valid options?
        opt.valid_for_db()?;
        // Create a new iterator
        let mut i = Iterator::new();
        // Assign the statement
        let stm = Statement::from(self);
        // Ensure futures are stored
        let opt = &opt.new_with_futures(false);
        // Check if there is a timeout
        let ctx = match self.timeout.as_ref() {
            Some(timeout) => {
                let mut ctx = MutableContext::new(ctx);
                ctx.add_timeout(*timeout.0)?;
                ctx.freeze()
            }
            None => ctx.clone(),
        };
        // Loop over the delete targets
        for w in self.what.0.iter() {
            let v = w.compute(stk, &ctx, opt, doc).await?;
            i.prepare(&stm, v).map_err(|e| match e {
                Error::InvalidStatementTarget { value: v } => Error::DeleteStatement { value: v },
                e => e,
            })?;
        }
        // Process the statement
        let res = i.output(stk, &ctx, opt, &stm).await?;
        // Catch statement timeout
        if ctx.is_timedout() {
            return Err(Error::QueryTimedout);
        }
        // Output the results
        match res {
            // This is a single record result
            Value::Array(mut a) if self.only => match a.len() {
                // There was exactly one result
                1 => Ok(a.remove(0)),
                // There were no results
                _ => Err(Error::SingleOnlyOutput),
            },
            // This is standard query result
            v => Ok(v),
        }
    }
}

impl fmt::Display for DeleteStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DELETE")?;
        if self.only {
            f.write_str(" ONLY")?
        }
        write!(f, " {}", self.what)?;
        if let Some(ref v) = self.cond {
            write!(f, " {v}")?
        }
        if let Some(ref v) = self.output {
            write!(f, " {v}")?
        }
        if let Some(ref v) = self.timeout {
            write!(f, " {v}")?
        }
        if self.parallel {
            f.write_str(" PARALLEL")?
        }
        Ok(())
    }
}
