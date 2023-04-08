#[derive(Clone, Copy)]
pub enum ReplaceMethod {
	/// Convert the token into a css var() statement, pointing to a css variable somewhere else in the system.
    CssVariables,
	/// Get the inner token value, this technically recurses until it finds the deepest static value. (i.e. not a handlebar reference to another token)
    StaticValues,
}