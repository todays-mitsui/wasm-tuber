use crate::context::Context;
use crate::style::ECMAScriptStyle;
use std::fmt::Display;

impl Display for ECMAScriptStyle<'_, Context> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut context = self.0.iter().collect::<Vec<_>>();

        context.sort_by(|l, r| {
            let l_name = l.0.as_str();
            let r_name = r.0.as_str();
            l_name.cmp(r_name)
        });

        write!(
            f,
            "{}",
            context
                .iter()
                .map(|(_, func)| ECMAScriptStyle(func).to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::func;
    use rand::seq::SliceRandom;

    #[test]
    fn test_to_string() {
        let mut functions = [
            func::new("i", vec!["x"], "x"),
            func::new("k", vec!["x", "y"], "x"),
            func::new("K", vec!["x", "y"], "x"),
            func::new("l", vec!["x", "y"], "x"),
        ];

        // functions を事前にシャッフルしてから Context を作る
        // これによって Context の印字が Func の順序依存でないことを確かめる

        let mut rng = rand::thread_rng();
        functions.shuffle(&mut rng);

        let context = Context::from(functions.to_vec());

        assert_eq!(
            ECMAScriptStyle(&context).to_string(),
            "
                K(x, y) = x\n\
                i(x) = x\n\
                k(x, y) = x\n\
                l(x, y) = x
            "
            .trim()
        );
    }
}
