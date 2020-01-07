use crate::parser::identifiers::*;
#[test]
fn can_parse_lc_ident_full() {
  assert_eq!(
    lc_ident_full("decryptedMessage#1f814f1f"),
    Ok((
      "",
      (
        String::from("decryptedMessage"),
        None,
        Some(String::from("1f814f1f"))
      )
    ))
  )
}
