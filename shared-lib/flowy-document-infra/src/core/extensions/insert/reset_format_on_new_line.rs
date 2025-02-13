use crate::{core::extensions::InsertExt, util::is_newline};
use lib_ot::core::{AttributeKey, Attributes, CharMetric, Delta, DeltaBuilder, DeltaIter, NEW_LINE};

pub struct ResetLineFormatOnNewLine {}
impl InsertExt for ResetLineFormatOnNewLine {
    fn ext_name(&self) -> &str { std::any::type_name::<ResetLineFormatOnNewLine>() }

    fn apply(&self, delta: &Delta, replace_len: usize, text: &str, index: usize) -> Option<Delta> {
        if !is_newline(text) {
            return None;
        }

        let mut iter = DeltaIter::new(delta);
        iter.seek::<CharMetric>(index);
        let next_op = iter.next_op()?;
        if !next_op.get_data().starts_with(NEW_LINE) {
            return None;
        }

        let mut reset_attribute = Attributes::new();
        if next_op.get_attributes().contains_key(&AttributeKey::Header) {
            reset_attribute.delete(&AttributeKey::Header);
        }

        let len = index + replace_len;
        Some(
            DeltaBuilder::new()
                .retain(len)
                .insert_with_attributes(NEW_LINE, next_op.get_attributes())
                .retain_with_attributes(1, reset_attribute)
                .trim()
                .build(),
        )
    }
}
