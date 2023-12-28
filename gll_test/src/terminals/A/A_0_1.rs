#![allow(non_snake_case)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) struct A_0_1;
impl<'a> wagon_gll::Label<'a> for A_0_1 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::state::GLLState<'a>,
    ) -> Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)> {
        vec![(vec![state.get_label_by_uuid("B")], None),]
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "A_0_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::state::GLLState<'a>) {
        let l_did_a = if let Some(v) = state.get_ret_val(0usize) {
            v
        } else {
            state.restore_attribute(1usize)
        }
            .to_owned();
        let label = state.get_label(&wagon_ident::Ident::Unknown("B".to_string()));
        if state.test_next(label.clone()) {
            state
                .gss_pointer = state
                .create(
                    std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("A"),
                            state.get_rule("A_0"),
                            2usize,
                            0,
                            "A_0",
                        ),
                    ),
                    vec![l_did_a,],
                );
            label.code(state);
        } else {
            return;
        }
    }
    fn to_string(&self) -> &str {
        "B"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["B",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec!["$did_a",], vec!["$did_a",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::state::GLLState<'a>,
    ) -> Option<wagon_gll::value::Value<'a>> {
        unreachable!("Weight should never be evaluated for non-zero GLL blocks")
    }
}
