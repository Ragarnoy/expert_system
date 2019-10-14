struct Operation
{
    outcome: Result<Outcome, Error>,
    operator: Operators,
    facts: (String, String),
    reverse: (bool, bool),
}